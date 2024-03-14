pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}

# fn main(){}
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --略--
}

fn function2() -> io::Result<()> {
    // --略--
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --略--
}

fn function2() -> IoResult<()> {
    // --略--
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

use std::collections::HashMap;

use std::{cmp::Ordering, io};

use std::io::{self, Write};

use std::collections::*;