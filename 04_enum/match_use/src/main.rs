#[derive(Debug)]
enum UsState
{
    Alabama,
    Alaska,
    Hawaii,
    Washington,
}
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_and_option_usage()
{
    let five = Some(5);
    println!("{:?}", five);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);
}


fn match_more_usage()
{
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ is a wildcard that matches any value that doesn't match any of the other cases
    }
}

fn main()
{
    let coin = Coin::Quarter(UsState::Washington);
    println!("{}", value_in_cents(coin));
    match_and_option_usage();
    match_more_usage();
}
