/*
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Create a new, empty string
    let mut guess = String::new();// mut means mutable

    // Read the next line from standard input (stdin) and place it into the string we created
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");// expect is a method of io::Result. If io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect

    println!("You guessed: {guess}");
}
 */

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);// 也可以使用(1..101)迭代器：https://doc.rust-lang.org/std/ops/struct.Range.html

    // println!("The secret number is: {secret_number}"); // {} is a placeholder for the value of secret_number

    loop {
        println!("Please input your guess.");

        // Create a new, empty string
        let mut guess = String::new(); // mut means mutable

        // Read the next line from standard input (stdin) and place it into the string we created
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // expect is a method of io::Result. If io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect

        // Convert the string to a number
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catchall value
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/*
use rand::{thread_rng, Rng};
fn main(){
let mut rng = thread_rng();
let n: u32 = rng.gen_range(0, 10);
println!("{}", n);
let m: f64 = rng.gen_range(-40.0f64, 1.3e5f64);
println!("{}", m);} */
