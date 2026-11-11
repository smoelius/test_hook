use proc_macro::TokenStream;
use quote::quote;
use syn::{Block, FnModifiers, ItemFn, parse::Nothing, parse_macro_input};

#[proc_macro_attribute]
pub fn test(args: TokenStream, tokens: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);
    let ItemFn {
        attrs,
        vis,
        modifiers: FnModifiers { defaultness, .. },
        sig,
        block,
    } = parse_macro_input!(tokens as ItemFn);
    let Block {
        brace_token: _,
        stmts,
    } = *block;
    quote! {
        #[::core::prelude::v1::test]
        #(#attrs)*
        #vis #defaultness #sig {
            ::test_hook::__call_test_hook(crate::__test_hook__);

            #(#stmts)*
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn test_hook(args: TokenStream, tokens: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);
    let item = parse_macro_input!(tokens as ItemFn);
    let ident = &item.sig.ident;
    quote! {
        const __test_hook__: ::test_hook::__TestHook = #ident;

        #item
    }
    .into()
}
