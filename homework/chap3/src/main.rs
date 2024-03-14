// 摄氏温度与华氏温度的相互转换
use std::io;
fn temp_convert(){
    println!("Please input the temperature in Celsius");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read line");
    let celsius: f32 = match celsius.trim().parse(){
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let fahrenheit = celsius * 1.8 + 32.0;
    println!("The temperature in Fahrenheit is: {fahrenheit}");
}

// 生成第n个斐波那契数列
fn fibonacci(n: u32) -> u32{
    if n == 1 || n == 2{// || means or
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// 打印圣诞颂歌The Twelve Days of Christmas的歌词，并且使用函数来减少重复代码
fn christmas_song(){
    let mut day = 1;
    while day <= 12{
        println!("On the {day} day of Christmas my true love sent to me");
        if day == 1{
            println!("A partridge in a pear tree");
        } else if day == 2{
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 3{
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 4{
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 5{
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 6{
            println!("Six geese a laying");
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 7{
            println!("Seven swans a swimming");
            println!("Six geese a laying");
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 8{
            println!("Eight maids a milking");
            println!("Seven swans a swimming");
            println!("Six geese a laying");
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 9{
            println!("Nine ladies dancing");
            println!("Eight maids a milking");
            println!("Seven swans a swimming");
            println!("Six geese a laying");
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 10{
            println!("Ten lords a leaping");
            println!("Nine ladies dancing");
            println!("Eight maids a milking");
            println!("Seven swans a swimming");
            println!("Six geese a laying");
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else if day == 11{
            println!("Eleven pipers piping");
            println!("Ten lords a leaping");
            println!("Nine ladies dancing");
            println!("Eight maids a milking");
            println!("Seven swans a swimming");
            println!("Six geese a laying");
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
        } else {
            println!("Twelve drummers drumming");
            println!("Eleven pipers piping");
            println!("Ten lords a leaping");
            println!("Nine ladies dancing");
            println!("Eight maids a milking");
            println!("Seven swans a swimming");
            println!("Six geese a laying");
            println!("Five golden rings");
            println!("Four calling birds");
            println!("Three French hens");
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");            
        }
        day += 1;
    }
}

fn main() {
    temp_convert();
    let n = 10;
    let result = fibonacci(n);
    println!("The {n}th fibonacci number is: {result}");
    christmas_song();
}

