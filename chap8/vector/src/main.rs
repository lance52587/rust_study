fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1,2,3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1,2,3,4];
        // do stuff with v
    }// <- v goes out of scope and is freed here

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // 改写成if let
    if let Some(third) = v.get(2) {
        println!("The third element is {}", third);
    } else {
        println!("There is no third element.");
    }

    let mut v = vec![1,2,3,4,5];

    let first = &v[0];

    //v.push(6); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

    println!("The first element is: {}", first);

    let v = vec![100,32,57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}