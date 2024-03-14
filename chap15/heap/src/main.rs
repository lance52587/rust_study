
enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}// 暂时无法编译通过：---- recursive without indirection，错误提示信息指出这个类型拥有无限大小

use crate::List::{Cons, Nil};

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}// 枚举类型的每一个成员都是一个单独的类型

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}// 定义一个泛型结构体

use std::ops::Deref;

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}

fn hello(name: &str){
    println!("Hello, {}!", name);
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);

    let y = Box::new(x);
    assert_eq!(5,*y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}