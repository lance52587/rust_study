fn main() {
    const MAX_POINTS: u32 = 100_000;

    // let x = 5;
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len();// error[E0308]: mismatched types    

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is {c}");
    println!("The value of z is {z}");
    println!("The value of heart_eyed_cat is {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x, y, z is {x}, {y}, {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    another_function(5,6);

    // let x = (let y = 6); // error[E0308]: mismatched types 语句和表达式的区别：语句不返回值，表达式返回值，let y = 6是语句，不能赋值给x

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    let x = five();
    println!("The value of x is {x}");
}

fn another_function(x: i32, y: i32){
    println!("Another function.");
    println!("The value of x, y is {x}, {y}");
}

fn five() -> i32 {
    5
}