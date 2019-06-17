use heck::*;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Lifetime};

pub fn make(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(item as Maybe);
    let t = quote!(#parsed);
    dbg!(format!("{:#}", t));
    proc_macro::TokenStream::from(t)
}

pub(crate) struct Maybe {
    name: Ident,
}

impl Parse for Maybe {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self { name: input.parse()? })
    }
}

impl ToTokens for Maybe {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.name.to_string().to_camel_case();

        let maybe_ident = Ident::new(&format!("Maybe{}", ident).to_camel_case(), Span::call_site());

        let is_ident_fn = Ident::new(&format!("is_{}", ident).to_snake_case(), Span::call_site());
        let is_ident_mut_fn = Ident::new(&format!("is_{}_mut", ident).to_snake_case(), Span::call_site());

        let static_ = Lifetime::new("'static", Span::call_site());

        let ident = Ident::new(&ident.to_camel_case(), Span::call_site());

        let expr = quote! {
            pub trait #maybe_ident: #static_ {
                fn #is_ident_fn(&self) -> Option<&dyn #ident> {
                    None
                }
                fn #is_ident_mut_fn(&mut self) -> Option<&mut dyn #ident> {
                    None
                }
            }
        };
        expr.to_tokens(tokens);
    }
}
