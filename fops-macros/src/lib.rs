mod opcodes;

use proc_macro::TokenStream;

#[proc_macro]
pub fn opcodes(item: TokenStream) -> TokenStream {
    opcodes::opcodes(item)
}
