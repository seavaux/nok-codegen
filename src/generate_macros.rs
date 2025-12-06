use crate::parse_args::{parse_args, Args};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_macros(args: TokenStream) -> TokenStream {
    let Args {
        payload_type: ty,
        export_globally,
    } = parse_args(args).unwrap();
    let (ty, export, allow_unused) = if export_globally {
        (
            quote! { $crate::#ty },
            Some(quote! { #[macro_export] }),
            None,
        )
    } else {
        (ty, None, Some(quote! { #[allow(unused)] }))
    };
    let crate_wide_export = if !export_globally {
        Some(quote! {
            pub(crate) use error;
            pub(crate) use err;
            pub(crate) use to_err_msg;
            pub(crate) use to_err_msg_dbg;
            pub(crate) use map_err;
            pub(crate) use map_err_dbg;
            pub(crate) use replace;
            pub(crate) use replace_err;
            pub(crate) use verify;
            pub(crate) use ok_or;
        })
    } else {
        None
    };
    quote! {
        #allow_unused
        #export
        macro_rules! error {
            () => {
                nok::error_t!(#ty)
            };
            ($($arg:tt)*) => {
                nok::error_t!(#ty, $($arg)*)
            };
        }
        #allow_unused
        #export
        macro_rules! err {
            () => {
                nok::err_t!(#ty)
            };
            ($($arg:tt)*) => {
                nok::err_t!(#ty, $($arg)*)
            };
        }
        #allow_unused
        #export
        macro_rules! to_err_msg {
            () => {
                nok::to_err_msg_t!(#ty)
            };
            ($payload:expr) => {
                nok::to_err_msg_t!(#ty, $payload)
            };
        }
        #allow_unused
        #export
        macro_rules! to_err_msg_dbg {
            () => {
                nok::to_err_msg_dbg_t!(#ty)
            };
            ($payload:expr) => {
                nok::to_err_msg_dbg_t!(#ty, $payload)
            };
        }
        #allow_unused
        #export
        macro_rules! map_err {
            ($err:expr) => {
                nok::map_err_t!(#ty, $err)
            };
            ($err:expr, $payload:expr) => {
                nok::map_err_t!(#ty, $err, $payload)
            };
        }
        #allow_unused
        #export
        macro_rules! map_err_dbg {
            ($err:expr) => {
                nok::map_err_dbg_t!(#ty, $err)
            };
            ($err:expr, $payload:expr) => {
                nok::map_err_dbg_t!(#ty, $err, $payload)
            };
        }
        #allow_unused
        #export
        macro_rules! replace {
            () => {
                nok::replace_t!(#ty)
            };
            ($($arg:tt)*) => {
                nok::replace_t!(#ty, $($arg)*)
            };
        }
        #allow_unused
        #export
        macro_rules! replace_err {
            ($err:expr) => {
                nok::replace_err_t!(#ty, $err)
            };
            ($err:expr, $($arg:tt)*) => {
                nok::replace_err_t!(#ty, $err, $($arg)*)
            };
        }
        #allow_unused
        #export
        macro_rules! verify {
            ($statement:expr) => {
                nok::verify_t!(#ty, $statement)
            };
            ($statement:expr, $($arg:tt)*) => {
                nok::verify_t!(#ty, $statement, $($arg)*)
            };
        }
        #allow_unused
        #export
        macro_rules! ok_or {
            ($opt:expr) => {
                nok::ok_or_t!(#ty, $opt)
            };
            ($opt:expr, $($arg:tt)*) => {
                nok::ok_or_t!(#ty, $opt, $($arg)*)
            };
        }

        #crate_wide_export
    }
}
