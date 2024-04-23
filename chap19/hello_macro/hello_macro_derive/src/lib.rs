extern crate proc_macro;// extern和use的区别是什么？ extern是引入外部库，use是引入模块。库和模块的区别是什么？库是一个可执行的程序，模块是一个库的一部分

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
    //q: quote!和into()分別是什么意思？
    //a: quote!是一个宏，它将代码转换为一个TokenStream，而into()是将TokenStream转换为TokenStream类型的返回值
}

#[proc_macro_derive(HelloMacro)]// proc_macro_derive是一个宏，HelloMacro是宏的名字
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将 Rust 代码解析为语法树以便进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}