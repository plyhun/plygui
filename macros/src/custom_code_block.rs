use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, token, Ident, Token};

pub struct CodeBlock {
	pub name: Ident,
	_colon: Token![:],
	_brace: token::Brace,
    pub custom: proc_macro2::TokenStream,
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

pub struct Custom {
	pub block1: Option<CodeBlock>,
    pub block2: Option<CodeBlock>,
    pub block3: Option<CodeBlock>,
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

