use proc_macro::TokenStream;
use syn::parse_macro_input;

mod soft_delete;

#[proc_macro_derive(SoftDeleteEntity)]
pub fn derive_soft_delete(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    soft_delete::expand(input).into()
}
