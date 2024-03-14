use trait2::Draw;
use trait2::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
    }
}

pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen3 {
    // pub components: Vec<Box<dyn Clone>>,//  `Clone` cannot be made into an object
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // 尝试使用一个没有实现Draw的类型
    let screen2 = Screen {
        components: vec![Box::new(String::from("Hi"))],// error: the trait `Draw` is not implemented for `std::string::String`
    };

    screen2.run();
}
