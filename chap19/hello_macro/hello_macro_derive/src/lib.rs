extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;// identifier，也就是标识符的意思，这里指DeriveInput的标识符
    let gen = quote! {
        impl HelloMacro for #name {// 为 Pancakes 实现 HelloMacro trait，#表示变量名，在rust中#的用法有：#name表示变量名，#(name)*表示变量名的重复
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将 Rust 代码解析为语法树以便进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}