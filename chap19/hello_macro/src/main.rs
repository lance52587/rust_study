use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}

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