use prettyplease::unparse;
use quote::quote;
use syn::parse2;

#[test]
fn test_generate_macros() {
    let actual = crate::generate_macros::generate_macros(
        quote! { payload_type: Payload, export_globally: true },
    );
    let expected = quote! {
        #[macro_export]
        macro_rules! error {
            () => {
                nok::error_t!($crate::Payload)
            };
            ($($arg:tt)*) => {
                nok::error_t!($crate::Payload, $($arg)*)
            };
        }
        #[macro_export]
        macro_rules! err {
            () => {
                nok::err_t!($crate::Payload)
            };
            ($($arg:tt)*) => {
                nok::err_t!($crate::Payload, $($arg)*)
            };
        }
        #[macro_export]
        macro_rules! to_err_msg {
            () => {
                nok::to_err_msg_t!($crate::Payload)
            };
            ($payload:expr) => {
                nok::to_err_msg_t!($crate::Payload, $payload)
            };
        }
        #[macro_export]
        macro_rules! to_err_msg_dbg {
            () => {
                nok::to_err_msg_dbg_t!($crate::Payload)
            };
            ($payload:expr) => {
                nok::to_err_msg_dbg_t!($crate::Payload, $payload)
            };
        }
        #[macro_export]
        macro_rules! map_err {
            ($err:expr) => {
                nok::map_err_t!($crate::Payload, $err)
            };
            ($err:expr, $payload:expr) => {
                nok::map_err_t!($crate::Payload, $err, $payload)
            };
        }
        #[macro_export]
        macro_rules! map_err_dbg {
            ($err:expr) => {
                nok::map_err_dbg_t!($crate::Payload, $err)
            };
            ($err:expr, $payload:expr) => {
                nok::map_err_dbg_t!($crate::Payload, $err, $payload)
            };
        }
        #[macro_export]
        macro_rules! replace {
            () => {
                nok::replace_t!($crate::Payload)
            };
            ($($arg:tt)*) => {
                nok::replace_t!($crate::Payload, $($arg)*)
            };
        }
        #[macro_export]
        macro_rules! replace_err {
            ($err:expr) => {
                nok::replace_err_t!($crate::Payload, $err)
            };
            ($err:expr, $($arg:tt)*) => {
                nok::replace_err_t!($crate::Payload, $err, $($arg)*)
            };
        }
        #[macro_export]
        macro_rules! verify {
            ($statement:expr) => {
                nok::verify_t!($crate::Payload, $statement)
            };
            ($statement:expr, $($arg:tt)*) => {
                nok::verify_t!($crate::Payload, $statement, $($arg)*)
            };
        }
        #[macro_export]
        macro_rules! ok_or {
            ($opt:expr) => {
                nok::ok_or_t!($crate::Payload, $opt)
            };
            ($opt:expr, $($arg:tt)*) => {
                nok::ok_or_t!($crate::Payload, $opt, $($arg)*)
            };
        }
    };

    let actual = unparse(&parse2(actual).unwrap());
    let expected = unparse(&parse2(expected).unwrap());
    assert_eq!(actual, expected);
}
