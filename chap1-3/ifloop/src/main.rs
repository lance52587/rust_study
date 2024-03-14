fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0{
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition{
        5
    } else {
        6
        // "six"// error[E0308]: `if` and `else` have incompatible types
    };
    println!("The value of number is: {number}");

    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut number = 3;
    while number != 0{
        println!("{number}!");
        number -= 1;
    }
    
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index != 5{
        // println!("the value is: {a[index]}");//
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter(){// for循环的优势：不会出现越界的情况，不需要手动的增加index，如while中写死的5
        println!("the value is: {element}");
    }

    for number in (1..4).rev(){// 1..4是一个迭代器，返回1,2,3
        println!("{number}!");
    }

}
