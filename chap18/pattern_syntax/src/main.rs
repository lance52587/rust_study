fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matchd, y = {:?}", y), // 这里的y是一个新的变量，而不是在程序起始处声明的存储了10的y。这个新的y的绑定能够匹配Some中的任意值，如果x不是Some(5)而是None，会匹配失败
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y); // match表达式创建出来的作用域会随着当前表达式的结束而结束，而它内部的y自然也无法幸免

    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // | 运算符可以用来匹配多个模式
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"), // ..= 运算符可以用来匹配一个范围
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // 解构结构体
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        // Message::Quit => {
        //     println!("The Quit variant has no data to destructure.")
        // },
        // Message::Move { x, y } => {
        //     println!("Move in the x direction {} and in the y direction {}", x, y)
        // },
        // Message::Write(text) => println!("Text message: {}", text),
        // Message::ChangeColor(r, g, b) => {
        //     println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        // },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    // 使用_忽略整个值
    foo(3, 4);

    // 使用嵌套的_忽略值的某些部分
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    // let new_setting_value = None;

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let _x = 5; // _x语法仍然将值绑定到了变量上
    let y = 10;

    println!("x = {}", _x);
    println!("y = {}", y);

    let s = Some(String::from("Hello!"));
    if let Some(ref _s) = s {// 使用ref关键字来创建一个引用
        println!("found a string");
        println!("{:?}", _s);
    }
    // 在这里继续使用 s
    println!("{:?}", s);

    if let Some(_s) = s {
        println!("found a string");
        println!("{:?}", _s);
    }

    // println!("{:?}", s); // 这里s的所有权已经被if let语句中的Some(_s)所获取，所以这里无法再使用s
    // println!("{:?}", _s); // 同样，这里也无法使用_s

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let numbers = (2, 4, 32);

    // match numbers {
    //     (.., second, ..) => {// can only be used once per tuple pattern
    //         println!("Some numbers: {}", second);
    //     }
    // }
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    // ChangeColor(i32, i32, i32),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
