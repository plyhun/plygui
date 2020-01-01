#![recursion_limit = "1024"]

extern crate proc_macro;

mod able_to;
mod as_into;
mod has;
mod maybe;
mod on;
mod define;
mod custom_code_block;

#[proc_macro]
pub fn able_to(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    able_to::make(item)
}

#[proc_macro]
pub fn has_reacted_set(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has::make(item, true, true)
}
#[proc_macro]
pub fn has_reacted(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has::make(item, true, false)
}
#[proc_macro]
pub fn has_settable(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has::make(item, false, true)
}
#[proc_macro]
pub fn has_private(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has::make(item, false, false)
}
#[proc_macro]
pub fn has_settable_reacted(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has::make(item, true, true)
}

#[proc_macro]
pub fn maybe(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    maybe::make(item)
}

#[proc_macro]
pub fn on(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    on::make(item)
}

#[proc_macro]
pub fn define_abstract(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	define::make(item, true)
}

#[proc_macro]
pub fn define(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	define::make(item, true)
}
