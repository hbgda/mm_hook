use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, token, parse::ParseStream};
use quote::quote;

#[proc_macro_derive(Component)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    quote! {
        impl Component for #ident {
            const NAME: &'static str = stringify!(#ident);
        }
    }.into()
}