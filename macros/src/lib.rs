#![recursion_limit = "1024"]

extern crate proc_macro;

mod able_to;
mod as_into;
mod has;
mod maybe;
mod on;

#[proc_macro]
pub fn able_to(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    able_to::make(item)
}

#[proc_macro]
pub fn has(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has::make(item, false)
}
#[proc_macro]
pub fn has_reacted(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has::make(item, true)
}

#[proc_macro]
pub fn maybe(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    maybe::make(item)
}

#[proc_macro]
pub fn on(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    on::make(item)
}
