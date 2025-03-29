use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Token};

#[proc_macro]
pub fn opcodes(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as OpcodesInput);

    let module_ident = input.module_ident;
    let entries = input.entries.iter();
    let count = entries.len();
    let instruction_names = input.entries.iter().map(|e| e.ident.to_string()).collect::<Vec<String>>();
    let instruction_sizes = input.entries.iter().map(|e| e.length as u8).collect::<Vec<u8>>();

    quote!(
        pub mod #module_ident {
            #(#entries)*

            pub const INSTRUCTION_NAMES: [&str; #count] = [#(#instruction_names,)*];
            pub const INSTRUCTION_LENGTH: [u8; #count] = [#(#instruction_sizes,)*];
        }
    ).into()
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

impl ToTokens for OpcodeEntry {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.ident;
        let code = &self.code;
        tokens.append_all(quote! {
            pub const #name: u8 = #code;
        })
    }
}
