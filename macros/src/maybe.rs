use heck::*;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident};

pub fn make(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(item as Maybe);
    let t = quote!(#parsed);
    dbg!(format!("{:#}", t));
    proc_macro::TokenStream::from(t)
}

pub(crate) struct Maybe {
    pub name: Ident,
}

impl Maybe {
    pub fn maybe_ident<S: AsRef<str>>(ident: S) -> Ident {
        Ident::new(&format!("Maybe{}", ident.as_ref()).to_camel_case(), Span::call_site())
    }
    pub fn is_ident<S: AsRef<str>>(ident: S) -> Ident {
        Ident::new(&format!("is_{}", ident.as_ref()).to_snake_case(), Span::call_site())
    }
    pub fn is_ident_mut<S: AsRef<str>>(ident: S) -> Ident {
        Ident::new(&format!("is_{}_mut", ident.as_ref()).to_snake_case(), Span::call_site())
    }
}

impl Parse for Maybe {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self { name: input.parse()? })
    }
}

impl ToTokens for Maybe {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.name.to_string().to_camel_case();

        let maybe_ident = Maybe::maybe_ident(ident);
        let is_ident_fn = Maybe::is_ident(ident);
        let is_ident_mut_fn = Maybe::is_ident_mut(ident);

        let ident = Ident::new(&ident.to_camel_case(), Span::call_site());

        let expr = quote! {
            pub trait #maybe_ident: 'static {
                fn #is_ident_fn(&self) -> Option<&dyn #ident> {
                    None
                }
                fn #is_ident_mut_fn(&mut self) -> Option<&mut dyn #ident> {
                    None
                }
            }
            
            impl<T: Member> #maybe_ident for T {
                #[inline]
                default fn #is_ident_fn(&self) -> Option<&dyn #ident> {
                    None
                }
                #[inline]
                default fn #is_ident_mut_fn(&mut self) -> Option<&mut dyn #ident> {
                    None
                }
            }
            impl<T: Member + #ident> #maybe_ident for T {
                #[inline]
                default fn #is_ident_fn(&self) -> Option<&dyn #ident> {
                    Some(self)
                }
                #[inline]
                default fn #is_ident_mut_fn(&mut self) -> Option<&mut dyn #ident> {
                    Some(self)
                }
            }
        };
        expr.to_tokens(tokens);
    }
}
