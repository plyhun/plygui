use crate::custom_code_block::Custom;
use crate::maybe::Maybe;

use heck::*;
use proc_macro2::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, parse_macro_input, token, Ident, Path, TypePath, PathArguments, PathSegment, Token, TraitBound, TraitBoundModifier, Type, TypeParamBound, TypeReference, TypeTraitObject};

use std::iter::FromIterator;

pub fn make(item: proc_macro::TokenStream, use_reactor: bool, use_get_set: bool) -> proc_macro::TokenStream {
    let mut parsed = parse_macro_input!(item as Has);
    parsed.use_reactor = use_reactor;
    parsed.use_get_set = use_get_set;
    let t = quote!(#parsed);
    dbg!(format!("{:#}", t));
    proc_macro::TokenStream::from(t)
}

struct HasReturnParams<'a> {
    params: &'a Punctuated<Type, Token![,]>,
    paren: token::Paren,
}
impl<'a> ToTokens for HasReturnParams<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.params.len() > 1 {
            self.paren.surround(tokens, |tokens| self.params.to_tokens(tokens));
        } else {
            self.params.to_tokens(tokens)
        }
    }
}

pub(crate) struct Has {
    name: Ident,
    _paren: token::Paren,
    params: Punctuated<Type, Token![,]>,
    _colon: Option<Token![:]>,
    extends: Option<Punctuated<Ident, Token![+]>>,
    _brace: Option<token::Brace>,
    custom: Option<Custom>,
    use_reactor: bool, 
    use_get_set: bool,
}

impl Parse for Has {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut extends_present = false;
        let content;
        let mut custom = None;
        Ok(Self {
            name: input.parse()?,
            _paren: parenthesized!(content in input),
            params: content.parse_terminated(Type::parse)?,
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
            use_reactor: true, 
            use_get_set: true,
        })
    }
}

impl ToTokens for Has {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.name.to_string().to_camel_case();

        let has_ident = Ident::new(&format!("Has{}", ident).to_camel_case(), Span::call_site());
        let has_ident_inner = Ident::new(&format!("Has{}Inner", ident).to_camel_case(), Span::call_site());

        let ident_fn = Ident::new(&format!("{}", ident).to_snake_case(), Span::call_site());
        let set_ident_fn = Ident::new(&format!("set_{}", ident).to_snake_case(), Span::call_site());
        let on_ident_set_fn = Ident::new(&format!("on_{}_set", ident).to_snake_case(), Span::call_site());

        let as_into = &crate::as_into::AsInto { ident_camel: &has_ident };

        let params = &self.params;
        let param_names = &(0..params.len()).map(|i| Ident::new(&format!("arg{}", i), Span::call_site())).collect::<Vec<_>>();
        let return_params = HasReturnParams {
            params: &self.params,
            paren: token::Paren { span: Span::call_site() },
        };

        let extends = self.extends.as_ref().map(|punct| punct.iter().map(|i| i.clone()).collect::<Vec<_>>()).unwrap_or(vec![]);
        let extends_inner = self
            .extends
            .as_ref()
            .map(|punct| punct.iter().map(|i| Ident::new(&format!("{}Inner", i.to_string().to_camel_case()), Span::call_site())).collect::<Vec<_>>())
            .unwrap_or(vec![]);
        let extends_base = &self
            .extends
            .as_ref()
            .map(|punct| punct.iter().map(|i| Ident::new(&format!("{}Base", i.to_string().to_camel_case()), Span::call_site())).collect::<Vec<_>>())
            .unwrap_or(vec![]);
        let extends_base_names = &self
            .extends
            .as_ref()
            .map(|punct| punct.iter().map(|i| Ident::new(i.to_string().to_snake_case().as_str(), Span::call_site())).collect::<Vec<_>>())
            .unwrap_or(vec![]);

        let on_ident = Ident::new(&ident.to_camel_case(), Span::call_site());
        
        let on = if self.use_reactor {
            let mut on = crate::on::On {
                name: on_ident.clone(),
                paren: token::Paren { span: Span::call_site() },
                params: Punctuated::new(),
                ret: crate::on::OnReturnParams::Single(
                    token::RArrow { spans: [Span::call_site(), Span::call_site()] },
                    Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter(
                                vec![PathSegment {
                                    ident: Ident::new("bool", Span::call_site()),
                                    arguments: PathArguments::None,
                                }]
                                .into_iter(),
                            ),
                        },
                    }),
                ),
                default_ret: Some(Ident::new("true", Span::call_site())),
            };
            let tto = TypeReference {
                and_token: token::And { spans: [Span::call_site()] },
                lifetime: None,
                mutability: Some(token::Mut { span: Span::call_site() }),
                elem: Box::new(Type::TraitObject(TypeTraitObject {
                    dyn_token: Some(token::Dyn { span: Span::call_site() }),
                    bounds: Punctuated::from_iter(
                        vec![TypeParamBound::Trait(TraitBound {
                            paren_token: None,
                            modifier: TraitBoundModifier::None,
                            lifetimes: None,
                            path: Path {
                                leading_colon: None,
                                segments: Punctuated::from_iter(
                                    vec![PathSegment {
                                        ident: has_ident.clone(),
                                        arguments: PathArguments::None,
                                    }]
                                    .into_iter(),
                                ),
                            },
                        })]
                        .into_iter(),
                    ),
                })),
            };
            on.params.push(Type::Reference(tto));
            self.params.iter().for_each(|i| on.params.push(i.clone()));
            
            let mut stream = TokenStream::new();
            on.to_tokens(&mut stream);
            stream
        } else { quote!{} };
        
        let reactor = if self.use_reactor && !self.use_get_set {
            quote! {
                fn #on_ident_set_fn(&mut self #(,#extends_base_names: &mut #extends_base)*, value: #return_params) -> bool;
            }
        } else { quote! {} };
        let getsetter = if self.use_get_set {
            quote! {
                fn #ident_fn(&self #(,#extends_base_names: &#extends_base)*) -> #return_params ;
                fn #set_ident_fn(&mut self #(,#extends_base_names: &mut #extends_base)* #(,#param_names: #params)*);
            }
        } else { quote! {} };

        let on_ident_fn = if self.use_reactor {
            let on_ident_fn = Ident::new(&format!("on_{}", ident).to_snake_case(), Span::call_site());
            let on_ident = Ident::new(&format!("On{}", ident).to_camel_case(), Span::call_site());

            quote! {
                fn #on_ident_fn(&mut self, callback: Option<#on_ident>);
            }
        } else { quote! {} };
        
        let on_ident_inner_fn = if self.use_reactor && self.use_get_set {
            let on_ident_fn = Ident::new(&format!("on_{}", ident).to_snake_case(), Span::call_site());
            let on_ident = Ident::new(&format!("On{}", ident).to_camel_case(), Span::call_site());

            quote! {
                fn #on_ident_fn(&mut self #(,#extends_base_names: &mut #extends_base)* ,callback: Option<#on_ident>);
            }
        } else { quote! {} };
        
        let (custom_trait, custom_inner) = {
        	let mut custom_trait = quote!{};
        	let mut custom_inner = quote!{};
        	if let Some(ref custom) = self.custom {
        		for block in [custom.blocks.get(0), custom.blocks.get(1)].iter() {
	        		if let Some(ref custom) = block {
		        		match custom.name.to_string().as_str() {
		        			"outer" => {
		        				custom_trait = custom.custom.clone();
		        			},
		        			"inner" => {
		        				custom_inner = custom.custom.clone();
		        			},
		        			_ => panic!("Unsupported custom block name :'{}'", custom.name),
		        		}
		        	}
        		}
        	}
        	(custom_trait, custom_inner)
        };

        let maybe = Maybe {
            name: has_ident.clone()
        };
        
        let expr = quote! {
            pub trait #has_ident: AsAny + 'static #(+#extends)* {
                fn #ident_fn(&self) -> #return_params ;
                fn #set_ident_fn(&mut self #(,#param_names: #params)*);
                #on_ident_fn

                #custom_trait
                #as_into
            }
            pub trait #has_ident_inner: 'static #(+#extends_inner)* {
                #getsetter
                #on_ident_inner_fn
                #reactor
                #custom_inner
            }

            #on
            #maybe
        };
        expr.to_tokens(tokens);
    }
}
