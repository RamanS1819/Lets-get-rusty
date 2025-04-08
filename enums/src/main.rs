enum IpAddrKind {
    V4(String),
    V6(String),
}

// Enum variants can store different types of data.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Some function");
    }
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}


// match expression
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // The enum is used to define a type that can be one of several variants.
    // In this case, the enum is used to define a type that can be either V4 or V6.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));


    // In rust the are no null values.
    // Instead, you can use the Option enum to represent a value that may or may not be present.
    /*
    enum Option<T> {
        Some(T),
        None,
    } 
    */
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x+y; /// this will not work because y is an Option and x is integer type
    // we can do this
    let sum = x + y.unwrap_or(0); // this will work because unwrap_or(0) will return 0 if y is None
    // However, using unwrap_or(0) is not the best way to handle the Option type.
    // Instead, you can use pattern matching to extract the value from the Option.
    // This is a common pattern in Rust, where you use pattern matching to handle different cases.
    let sum = match y {
        Some(y) => x + y,
        None => x,
    };



    // Match Expression
    // The match expression is used to match a value against a pattern and execute code based on the pattern.
    value_in_cents(Coin::Quarter(UsState::Alaska));

}

fn route(ip_kind: IpAddrKind) {
    // The function takes an argument of type IpAddrKind and does something with it.
    // In this case, the function is not implemented, but it could be used to route
    // network traffic based on the type of IP address.
}


// match expression
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
// The match expression is used to match the value of the coin against the different variants of the Coin enum.
// The match expression is exhaustive, meaning that all possible cases must be handled.
// If a case is not handled, the compiler will give an error.


// Combiniting match expression with Option enum
fn plus_one(x: Option<i32>) -> i32 {
    match x {
        Some(i) => i + 1,
        None => 0,
    }
}