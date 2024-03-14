fn main() {
    println!("Coin::Penny = {}",value_in_cents(Coin::Penny));
    println!("Coin::Nickel = {}",value_in_cents(Coin::Nickel));
    println!("Coin::Dime = {}",value_in_cents(Coin::Dime));
    println!("Coin::Quater = {}",value_in_cents(Coin::Quater(UsState::Alabama)));
    println!("Coin::Quater = {}",value_in_cents(Coin::Quater(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}",five);
    println!("six = {:?}",six);
    println!("none = {:?}",none);
    
    let some_u8_value = 0u8;
    match some_u8_value{
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _ => (),
        _ => println!("other"),
    }
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // --snip--
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    // Quater,
    Quater(UsState),
}

fn value_in_cents(coin:Coin) -> u32{
    match coin{
        Coin::Penny => {
            println!("Lucky Penny");
            1},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quater => 25,
        Coin::Quater(state) => {
            println!("State quarter from {:?}!",state);
            25
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}
