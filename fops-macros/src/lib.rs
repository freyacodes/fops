mod opcodes;
mod vm_test;

use proc_macro::TokenStream;

#[proc_macro]
pub fn opcodes(item: TokenStream) -> TokenStream {
    opcodes::opcodes(item)
}

#[proc_macro]
pub fn vm_test(item: TokenStream) -> TokenStream {
    vm_test::vm_test(item.into()).into()
}
