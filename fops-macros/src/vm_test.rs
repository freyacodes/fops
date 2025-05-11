use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};

pub(crate) fn vm_test(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as VmTest);
    let write_statements = input
        .inputs
        .iter()
        .map(|i| i.to_append_chunk_statement())
        .collect::<Vec<TokenStream>>();

    let assertion = match input.expected {
        Expected::Boolean(lit) => quote! {
            assert_eq!(crate::vm::value::Value::Bool(#lit), crate::vm::run(&chunk).unwrap());
        },
        Expected::Float(lit) => quote! {
            assert_eq!(crate::vm::value::Value::Number(#lit), crate::vm::run(&chunk).unwrap());
        },
        Expected::RuntimeError => quote! {
            crate::vm::tests::assert_runtime_error(crate::vm::run(&chunk));
        },
        Expected::Expression(expr) => quote! {
            assert_eq!(#expr, crate::vm::run(&chunk).unwrap());
        },
    };

    quote! {{
        let mut chunk = crate::bytecode::chunk::Chunk::new();
        #(#write_statements)*
        chunk.write0(OP_RETURN);
        #assertion
    }}.into()
}

struct VmTest {
    inputs: Vec<Input>,
    expected: Expected,
}

enum Input {
    Op(syn::Ident),
    Float(syn::LitFloat),
    FloatIdentifier(syn::Ident),
}

enum Expected {
    Boolean(syn::LitBool),
    Float(syn::LitFloat),
    Expression(syn::Expr),
    RuntimeError
}

impl Parse for VmTest {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut inputs: Vec<Input> = Vec::new();

        while !input.peek(Token![=>]) {
            if input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                if ident.clone().into_token_stream().to_string().starts_with("OP_") {
                    inputs.push(Input::Op(ident));
                } else {
                    inputs.push(Input::FloatIdentifier(ident));
                }
            } else if input.peek(syn::LitFloat) {
                inputs.push(Input::Float(input.parse()?));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        input.parse::<Token![=>]>()?;

        let expected = if input.peek(syn::LitFloat) {
            Expected::Float(input.parse()?)
        } else if input.peek(syn::LitBool) {
            Expected::Boolean(input.parse()?)
        } else if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            Expected::RuntimeError
        } else {
            Expected::Expression(input.parse()?)
        };

        Ok(VmTest { inputs, expected })
    }
}

impl Input {
    fn to_append_chunk_statement(&self) -> TokenStream {
        match self {
            Input::Op(ident) => quote! {
                chunk.write0(#ident);
            },
            Input::Float(lit) => quote! {
                chunk.write_constant_f64_0(#lit);
            },
            Input::FloatIdentifier(ident) => quote! {
                chunk.write_constant_f64_0(#ident);
            },
        }
    }
}
