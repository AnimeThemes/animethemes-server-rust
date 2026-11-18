use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;

    if ident == "ModelEx" {
        return TokenStream::new();
    }

    if ident != "Model" {
        return syn::Error::new_spanned(ident, "SoftDeleteEntity must be derived on Model")
            .to_compile_error();
    }

    quote! {
        impl crate::traits::soft_delete::SoftDeleteEntity for Entity {
            fn deleted_at_column() -> Self::Column {
                Column::DeletedAt
            }
        }
    }
}
