use crate::custom_code_block::Custom;
use crate::maybe::Maybe;

use heck::*;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, token, Ident, Token};

pub fn make(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(item as Define);
    let t = quote!(#parsed);
    dbg!(format!("{:#}", t));
    proc_macro::TokenStream::from(t)
}

pub struct Define {
	name: Ident,
	_colon: Option<Token![:]>,
    extends: Option<Punctuated<Ident, Token![+]>>,
    _brace: Option<token::Brace>,
    custom: Option<Custom>,
}

impl Parse for Define {
    fn parse(input: ParseStream) -> Result<Self> {
	    let mut extends_present = false;
	    let mut custom = None;
        Ok(Self {
            name: input.parse()?,
            _colon: {
                let lookahead = input.lookahead1();
                if lookahead.peek(Token![:]) {
                    extends_present = true;
                    Some(input.parse()?)
                } else {
                    None
                }
            },
            extends: if extends_present {
                let mut extends: Punctuated<Ident, Token![+]> = Punctuated::new();
                loop {
                    extends.push_value(input.parse()?);
                    if input.peek(token::Brace) || input.is_empty() {
                        break;
                    }
                    extends.push_punct(input.parse()?);
                    if input.peek(token::Brace) || input.is_empty() {
                        break;
                    }
                }
                Some(extends)
            } else {
                None
            },
            _brace: {
                let lookahead = input.lookahead1();
                if lookahead.peek(token::Brace) {
                    let content;
                    let brace = braced!(content in input);
                    custom = Some(content);
                    Some(brace)
                } else {
                    None
                }
            },
            custom: custom.map(|content| content.parse().unwrap()),
	    })
    }
}

impl ToTokens for Define {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = Ident::new(&self.name.to_string().to_camel_case(), Span::call_site());
        
        let ident_base = Ident::new(&format!("{}Base", ident).to_camel_case(), Span::call_site());
        let a_ident = Ident::new(&format!("A{}", ident).to_camel_case(), Span::call_site());
        let ident_inner = Ident::new(&format!("{}Inner", ident).to_camel_case(), Span::call_site());
        
        let as_into = &crate::as_into::AsInto { ident_camel: &ident };

		let extends = &self.extends.as_ref().map(|punct| punct.iter().map(|i| i.clone()).collect::<Vec<_>>()).unwrap_or(vec![]);
        let extends_inner = self
            .extends
            .as_ref()
            .map(|punct| punct.iter().map(|i| Ident::new(&format!("{}Inner", i.to_string().to_camel_case()), Span::call_site())).collect::<Vec<_>>())
            .unwrap_or(vec![]);

        let (type_base, custom_base, custom_trait, custom_inner, constructor, custom_extends, custom_implements) = {
        	let mut should_have_constructor = true;
        	let mut type_base = quote!{};
        	let mut constructor = quote!{};
        	let mut custom_base = quote!{};
        	let mut custom_trait = quote!{};
        	let mut custom_inner = quote!{};
        	let mut custom_extends = quote!{};
        	let mut custom_implements = quote!{};
        	if let Some(ref custom) = self.custom {
        	    for block in [custom.blocks.get(0), custom.blocks.get(1), custom.blocks.get(2), custom.blocks.get(3), custom.blocks.get(4)].iter() {
	        		if let Some(custom) = block {
		        		match custom.name.to_string().as_str() {
		        			"base" => {
		        				type_base = quote! { pub base: #ident_base, };
		        				let custom = &custom.custom;
		        				custom_base = quote! {
		        					#[repr(C)]
			        				pub struct #ident_base {
										#custom
									}
		        				};
		        				should_have_constructor = false;
		        			},
		        			"outer" => {
		        				custom_trait = custom.custom.clone();
		        			},
		        			"inner" => {
		        				custom_inner = custom.custom.clone();
		        			},
		        			"extends" => {
		        			    let custom = &custom.custom;		        				
		        			    custom_extends = quote!{ + #custom };
		        			},
		        			"implements" => {
    		        			let custom = &custom.custom;		        				
		        			    custom_implements = quote!{ + #custom };
		        			},
		        			_ => panic!("Unknown custom block name :'{}'", custom.name),
		        		}
		        	}
        		}
        	}
        	if should_have_constructor {
            	constructor = quote! {
                	impl<T: #ident_inner> #a_ident<T> {
                        #[inline]
                        pub fn with_inner(inner: T) -> Self {
                            #a_ident { inner }
                        }
            		}
            	};
        	}
        	(type_base, custom_base, custom_trait, custom_inner, constructor, custom_extends, custom_implements)
        };
        
        let maybe = Maybe {
            name: ident.clone()
        };
        /*let maybe_ident = Maybe::maybe_ident(&ident.to_string());
        let is_ident_fn = Maybe::is_ident(&ident.to_string());
        let is_ident_mut_fn = Maybe::is_ident_mut(&ident.to_string());
        let ident2 = ident.clone();*/
        
		let expr = quote! {
			#custom_base
			
			#[repr(C)]
			pub struct #a_ident<T: #ident_inner> {
				#type_base
				pub inner: T
			}
			
			pub trait #ident: 'static #(+#extends)* #custom_extends {
            	#custom_trait
                #as_into
            }
            pub trait #ident_inner: 'static #(+#extends_inner)* #custom_implements {
            	#custom_inner
            }
            
            impl<T: #ident_inner> HasInner for #a_ident<T> {
                type I = T;
            
                fn inner(&self) -> &Self::I {
                    &self.inner
                }
                fn inner_mut(&mut self) -> &mut Self::I {
                    &mut self.inner
                }
                fn into_inner(self) -> Self::I {
                    self.inner
                }
            }
            
            #constructor
            #maybe
        };
        expr.to_tokens(tokens);
        
        /*for extend in extends {
            let expr = quote! {
                impl<T: #extend> #maybe_ident for T {
                    #[inline]
                    default fn #is_ident_fn(&self) -> Option<&dyn #ident> {
                        None
                    }
                    #[inline]
                    default fn #is_ident_mut_fn(&mut self) -> Option<&mut dyn #ident2> {
                        None
                    }
                }
            };
            expr.to_tokens(tokens);
        }*/
    }
}
