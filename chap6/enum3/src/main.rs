fn main() {
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    if let Some(3) = some_u8_value{
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Quater(UsState::Alaska);
    // match coin{
    //     Coin::Quater(state) => println!("State quarter from {:?}!",state),
    //     _ => count += 1,
    // }
    if let Coin::Quater(state) = coin{
        println!("State quarter from {:?}!",state);
    }else{
        count += 1;
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
