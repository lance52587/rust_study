use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self{
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest member is x = {}", self.x);
        }else{
            println!("The largest member is y = {}", self.y);
        }
    }
}

// impl<T: Display> ToString for T{
//     fn to_string(&self) -> String{
//         format!("({})", self)
//     }
// }



fn main() {
    let s = 3.to_string();
    println!("s = {}", s);

    let pair = Pair::new(3, 4);
    pair.cmp_display();
}
