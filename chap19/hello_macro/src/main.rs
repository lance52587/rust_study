use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
struct Pancakes;
// DeriveInput{
//     // --snip--
//     pub ident: Ident{
//         ident: "Pancakes",
//         span: #0 bytes(95..103),
//     },
//     data: Struct(
//         DataStruct{
//             struct_token: Struct,
//             fields: Unit,
//             semi
//         },
//     ),
// }

// 属性宏
// #[route(GET, "/")]
// fn index(){}

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item:TokenStream) -> TokenStream{}

// 函数宏
// let sql = sql!(SELECT * FROM posts WHERE id=1);
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream{}


impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
