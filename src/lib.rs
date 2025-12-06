use proc_macro::TokenStream;
mod generate_macros;
mod parse_args;

#[cfg(test)]
mod test;

/// Generate macros with an embedded `Payload` type.
/// Each generated macro has a counterpart in the `nok` crate that requires the `Payload` type to
/// be provided as argument.
///
/// Generated macros and their counterparts:
/// - `error` -> `nok::error_t`
/// - `err` -> `nok::err_t`
/// - `to_err_msg` -> `nok::to_err_msg_t`
/// - `to_err_msg_dbg` -> `nok::to_err_msg_dbg`
/// - `map_err` -> `nok::map_err_t`
/// - `map_err_dbg` -> `nok::map_err_dbg_t`
/// - `replace` -> `nok::replace_t`
/// - `replace_err` -> `nok::replace_err_t`
/// - `verify` -> `nok::verify_t`
/// - `ok_or` -> `nok::ok_or_t`
///
/// Arguments are expected as a list of `<ident>: <expr>`'s separated by `,`.
///
/// (required) payload_type: <tokens>,
/// (optional) export_globally: true|false,
///     if true
///         then each macro gets prefixed with `#[macro_export]`
///         else `pub(crate) use <macro>;` is used
#[proc_macro]
pub fn generate_macros(input: TokenStream) -> TokenStream {
    generate_macros::generate_macros(input.into()).into()
}
