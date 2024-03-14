struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data '{}'!",self.data);
    }
}

use std::mem::drop;
fn main() {
    let c = CustomSmartPointer{data :String::from("my stuff")};
    let d = CustomSmartPointer{data :String::from("other stuff")};
    println!("CustomSmartPointer created.");
    // d.drop();// explicit destructor calls not allowed
    // println!("CustomSmartPointer dropped before the end of main.")
    drop(d);
    println!("CustomSmartPointer dropped before the end of main.");
}
