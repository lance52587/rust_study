fn main() {
    // if let Some(x) = some_option_value{
    //     println!("{}", x);
    // }

    if let x = 5{
        println!("{}", x);// this pattern will always match, so the `if let` is useless
    }
}
