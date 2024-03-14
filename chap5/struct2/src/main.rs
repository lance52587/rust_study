fn main() {
    let weight1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.",
             area(weight1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.",
             area2(rect1));

    let rect2 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.",
             area3(&rect2));
    // println!("rect2 is {}", rect2);// error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    println!("rect2 is {:?}", rect2);// error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]// error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}