static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe{
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    add_to_count(3);

    unsafe{
        println!("COUNTER: {}", COUNTER);
    }

    println!("name is: {}", HELLO_WORLD);
}