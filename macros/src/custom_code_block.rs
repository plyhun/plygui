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
	pub blocks: Vec<CodeBlock>,
}
impl Parse for Custom {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut custom = Self { blocks: vec![] };
		loop {
    		let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                custom.blocks.push(input.parse()?);
            } else {
                break;
            }
		}
		Ok(custom)
	}
}

