fn main() {
    //
    // variable : mutable by adding 'mut' keyword, can be declared in any scope
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constant : immutable, can be declared in any scope
    const MAX_POINTS: u32 = 100 * 2;  // _ can be used to improve readability
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // shadowing
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);
    let y = "hello";
    println!("String y is: {}", y);


    // data types
    /// scalar types
    // integer -----  signed integers can be =ve as well as -ve,,, but unsigned integer can only be +ve
    let a = 98_222; // decimal
    let b = 0xff;  // hexadecimal
    let c = 0o77;  // octal
    let d = 0b1111_0000;  // binary
    let e = b'A';  // byte (u8 only)
    //interger overflow
    // let f: u8 = 256;  // error: literal out of range for u8
    // let g: i8 = 128;  // error: literal out of range for i8

    // floating point numbers
    let h = 2.0;  // f64
    let i: f32 = 3.0;  // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false;

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    /// compound types
    /// tuple : tuple is a collection of values of different types
    let tup = ("Less go", 5);
    let (x, y) = tup;
    let five = tup.1;

    /// array : array is a collection of values of same type
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];

    // arragys can also be created like this: create an array with 8 values all set to 0
    let arr = [0; 8];
    println!("The value of arr is: {:?}", arr);




    // functions
    another_function(18, 19);
    let sum = another_function(18, 19);
    println!("The sum is: {}", sum);




    // control flow
    let number = 5;

    if number <10 {
        println!("The number is less than 10");
    } else if number > 10 && number < 20 {
        println!("The number is greater than 10 but less than 20");
    } else {
        println!("The number is greater than or equal to 10");
    }

    //
    let condition = true;
    let number = if condition { 5 } else { 6 };



    // loop -> 3 types
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("The result is: {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }

}

fn another_function(x: i32, y: i32) -> i32 {
    // in rust we could think of a block of code as an expression or a statement
    // expression : return a value  eg. let sum = x+y;       so x+y is an expression
    //, statement : do not return a value   eg. let x = 5;    so let x = 5; is a statement
    // rust is an expression based language
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
