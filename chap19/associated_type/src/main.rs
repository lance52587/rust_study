pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display{
    fn outline_print(&self){
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point{}// the trait `std::fmt::Display` is not implemented for `Point`

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)// write表示将格式化的字符串写入到f中，定义了outline_print中调用的to_string
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 假如我们希望新类型具有内部类型的所有方法，那么我们也可以为Wrapper实现Deref trait（在第15章的“通过Deref trait将智能指针视作常规引用”一节曾经讨论过这一技术）来直接返回内部的类型。
// 通过实现Deref trait，我们可以编写一个返回内部类型的实例的方法，这样我们就可以调用内部类型的方法了。
use std::ops::Deref;

impl Deref for Wrapper{
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    // println!("w = {:?}", w);// error: the trait `std::fmt::Debug` is not implemented for `Wrapper`

    // 使用内部类型的方法
    let vec = w.deref();
    println!("vec = {:?}", vec);

    let p = Point { x: 1, y: 2 };
    p.outline_print();

    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let person = Human;
    Pilot::fly(&person);//This is your captain speaking.
    Wizard::fly(&person);//Up!
    // person.fly();//*waving arms furiously*
    Human::fly(&person);//*waving arms furiously*

    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
}
