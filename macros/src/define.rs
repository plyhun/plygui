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

struct CodeBlock {
	name: Ident,
	_colon: Token![:],
	_brace: token::Brace,
    custom: proc_macro2::TokenStream,
    _comma: Option<Token![,]>
}
impl Parse for CodeBlock {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;
		Ok(Self {
			name: input.parse()?,
			_colon: input.parse()?,
			_brace: braced!(content in input),
			custom: content.parse().unwrap(),
			_comma: input.parse()?
		})
	}
}

struct Custom {
	block1: Option<CodeBlock>,
    block2: Option<CodeBlock>,
    block3: Option<CodeBlock>,
}
impl Parse for Custom {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			block1: {
				let lookahead = input.lookahead1();
                if lookahead.peek(Ident) {
                    Some(input.parse()?)
                } else {
                    None
                }
			},
			block2: {
				let lookahead = input.lookahead1();
                if lookahead.peek(Ident) {
                    Some(input.parse()?)
                } else {
                    None
                }
			},
			block3: {
				let lookahead = input.lookahead1();
                if lookahead.peek(Ident) {
                    Some(input.parse()?)
                } else {
                    None
                }
			},
		})
	}
}

pub(crate) struct Define {
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

		let extends = self.extends.as_ref().map(|punct| punct.iter().map(|i| i.clone()).collect::<Vec<_>>()).unwrap_or(vec![]);
        let extends_inner = self
            .extends
            .as_ref()
            .map(|punct| punct.iter().map(|i| Ident::new(&format!("{}Inner", i.to_string().to_camel_case()), Span::call_site())).collect::<Vec<_>>())
            .unwrap_or(vec![]);

        let (type_base, custom_base, custom_trait, custom_inner) = {
        	let mut type_base = quote!{};
        	let mut custom_base = quote!{};
        	let mut custom_trait = quote!{};
        	let mut custom_inner = quote!{};
        	if let Some(ref custom) = self.custom {
        		for block in [&custom.block1, &custom.block2, &custom.block3].iter() {
	        		if let Some(ref custom) = block {
		        		match custom.name.to_string().as_str() {
		        			"base" => {
		        				type_base = quote! { base: #ident_base, };
		        				let custom = custom.custom.clone();
		        				custom_base = quote! {
		        					#[repr(C)]
			        				pub struct #ident_base {
										#custom
									}
		        				};
		        			},
		        			"outer" => {
		        				custom_trait = custom.custom.clone();
		        			},
		        			"inner" => {
		        				custom_inner = custom.custom.clone();
		        			},
		        			_ => panic!("Unknown custom block name :'{}'", custom.name),
		        		}
		        	}
        		}
        	}
        	(type_base, custom_base, custom_trait, custom_inner)
        };
        
		let expr = quote! {
			#custom_base
			
			#[repr(C)]
			pub struct #a_ident<T: #ident_inner> {
				#type_base
				inner: T
			}
			
			pub trait #ident: 'static #(+#extends)* {
            	#custom_trait
                #as_into
            }
            pub trait #ident_inner: 'static #(+#extends_inner)* {
            	#custom_inner
            }
        };
        expr.to_tokens(tokens);
    }
}
