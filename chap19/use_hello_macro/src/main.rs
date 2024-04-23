use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream{
    input
}
fn main() {
    let sql = sql!(SELECT * FROM posts WHERE id=1);

    Pancakes::hello_macro();
}
