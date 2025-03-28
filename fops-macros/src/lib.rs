use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

#[proc_macro]
pub fn opcodes(item: TokenStream) -> TokenStream {
    let input = syn::parse::<OpcodesInput>(item).expect("Failed to parse");
    TokenStream::new()
}

struct OpcodesInput {
    module_ident: syn::Ident,
    _colon: Token![:],
    entries: Punctuated<OpcodeEntry, Token![,]>,
}

struct OpcodeEntry {
    code: syn::LitInt,
    ident: syn::Ident,
    length: usize,
}

mod kw {
    syn::custom_keyword!(len);
}

impl Parse for OpcodesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(OpcodesInput {
            module_ident: input.parse()?,
            _colon: input.parse()?,
            entries: input.parse_terminated(OpcodeEntry::parse, Token![,])?
        })
    }
}

impl Parse for OpcodeEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let code = input.parse()?;
        input.parse::<Token![=]>()?;
        let ident = input.parse()?;
        let size = if input.peek(kw::len) {
            input.parse::<kw::len>()?;
            input.parse::<syn::LitInt>()?.base10_parse()?
        } else {
            1
        };
        Ok(OpcodeEntry { code, ident, length: size })
    }
}
