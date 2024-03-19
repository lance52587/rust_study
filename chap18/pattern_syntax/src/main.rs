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
}
