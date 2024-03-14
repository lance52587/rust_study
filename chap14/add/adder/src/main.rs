use add_one;
// use rand;// no external crate 'rand'
fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!",num, add_one::add_one(num));
}
