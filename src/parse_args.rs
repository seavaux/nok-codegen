use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;

static EXPECTED: &'static str =
    "Expected '{ payload_type: <tokens>, should_export_globally: true|false }'";
static EXPECTED_AT_LEAST_PAYLOAD: &'static str = "Expected at least 'payload_type'";
static EXPECTED_EXPORT_GLOBALLY: &'static str = "Expected 'export_globally: true|false'";

type Res<T> = Result<T, &'static str>;

#[derive(Debug)]
pub struct Args {
    pub payload_type: TokenStream,
    pub export_globally: bool,
}

impl PartialEq for Args {
    fn eq(&self, other: &Self) -> bool {
        self.export_globally == other.export_globally
            && self.payload_type.to_string() == other.payload_type.to_string()
    }
}

pub fn parse_args(args: TokenStream) -> Res<Args> {
    use syn::{parse::Parser, punctuated::Punctuated, Expr, FieldValue, Token};
    let args = Punctuated::<FieldValue, Token![,]>::parse_separated_nonempty
        .parse2(args)
        .map_err(|_| EXPECTED)?;

    let mut args: HashMap<String, Expr> = args
        .into_pairs()
        .map(|x| {
            let kv = x.into_value();
            let key = match kv.member {
                syn::Member::Named(x) => x.to_string(),
                _ => return Err(EXPECTED),
            };
            let val = kv.expr;
            Ok((key, val))
        })
        .collect::<Res<HashMap<String, Expr>>>()?;

    let payload_type = args
        .remove(&"payload_type".to_string())
        .ok_or(EXPECTED_AT_LEAST_PAYLOAD)?
        .to_token_stream();

    let export_globally = args
        .remove(&"export_globally".to_string())
        .map(|x| {
            let x = x.to_token_stream().to_string();
            match x.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(EXPECTED_EXPORT_GLOBALLY),
            }
        })
        .transpose()?;
    Ok(Args {
        payload_type,
        export_globally: export_globally.unwrap_or(false),
    })
}

#[test]
fn test_parse_args() {
    use quote::quote;

    {
        let expected = Ok(Args {
            payload_type: quote! { Payload },
            export_globally: false,
        });
        let actual = parse_args(quote! { payload_type: Payload });
        assert_eq!(actual, expected);
    }

    {
        let expected = Ok(Args {
            payload_type: quote! { Payload },
            export_globally: false,
        });
        let actual = parse_args(quote! { payload_type: Payload, export_globally: false });
        assert_eq!(actual, expected);
    }

    {
        let expected = Ok(Args {
            payload_type: quote! { Payload },
            export_globally: true,
        });
        let actual = parse_args(quote! { payload_type: Payload, export_globally: true });
        assert_eq!(actual, expected);
    }

    {
        let expected = Err(EXPECTED);
        let actual = parse_args(quote! {});
        assert_eq!(actual, expected);
    }

    {
        let expected = Err(EXPECTED_AT_LEAST_PAYLOAD);
        let actual = parse_args(quote! { Payload });
        assert_eq!(actual, expected);
    }

    {
        let expected = Err(EXPECTED_EXPORT_GLOBALLY);
        let actual = parse_args(quote! { payload_type: tokens, export_globally: no });
        assert_eq!(actual, expected);
    }
}
