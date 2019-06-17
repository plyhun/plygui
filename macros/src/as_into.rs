use heck::*;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::Ident;

pub struct AsInto<'a> {
    pub ident_camel: &'a Ident,
}

impl<'a> ToTokens for AsInto<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident_camel;

        let as_ident = Ident::new(&format!("as_{}", ident).to_snake_case(), Span::call_site());
        let as_ident_mut = Ident::new(&format!("as_{}_mut", ident).to_snake_case(), Span::call_site());
        let into_ident = Ident::new(&format!("into_{}", ident).to_snake_case(), Span::call_site());

        let expr = quote! {
            fn #as_ident(&self) -> &dyn #ident;
            fn #as_ident_mut(&mut self) -> &mut dyn #ident;
            fn #into_ident(self: Box<Self>) -> Box<dyn #ident>;
        };
        expr.to_tokens(tokens)
    }
}
