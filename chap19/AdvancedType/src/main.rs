// let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
type Thunk = Box<dyn Fn() + Send + 'static>;
let f: Thunk = Box::new(|| println!("hi"));

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
fn takes_long_type(f: Thunk) {
    // --snip--
}

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
fn returns_long_type() -> Thunk {
    // --snip--
    Box::new(|| ())
}

use std::io::Error;
use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    // fn flush(&mut self) -> Result<(), Error>;
    fn flush(&mut self) -> Result<()>;

    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn bar() -> ! {
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),// panic!宏，返回值是!
        }
    }
}

// fn generic<T>(T: ?Sized) {
fn generic<T: Sized>(t: T) {
    // --snip--
}

fn generic

fn main() {
    // let s1: str = "Hello there!";// 应该为&str
    // let s2: str = "How's it going?";

    let guess: u32 = match guess.trim().parse() {// trim()去掉空格
        Ok(num) => num,
        Err(_) => continue,// 返回值是!
    };

    print!("forever ");
    loop {// loop是无限循环，返回值是!或者break
        print!("and ever ");
    }

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
