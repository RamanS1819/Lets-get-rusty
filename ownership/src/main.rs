fn main() {
    // ---- Ownership rules ----
    // 1. Each value in Rust has a variable that’s called its “owner”.
    // 2. A value can only have one owner at a time.
    // 3. When the owner of a value goes out of scope, the value will be dropped.
    // 4. If you assign a value to another variable, the first variable is no longer valid.

    let x = 5;
    let y = x;



    let s1 = String::from("hello");
    // let s2 = s1;  // println!("{}", s1); // This will cause a compile-time error because s1 is no longer valid
    let s2 = s1.clone(); // This creates a deep copy of s1

    println!("{}, world!", s1);


    // ---- ownerships and functions ----
    let s = String::from("hello");
    takes_ownership(s); // s is moved into the function
    // println!("{}", s); // This will cause a compile-time error because s is no longer valid

    let x = 5;
    makes_copy(x); // x is copied into the function (integers can make copy easily)
    println!("{}", x); // x is still valid here

    // giving and taking back values
    let s1 = gives_ownership(); // s1 is moved into the function
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into the function and then moved back to s3
    println!("s1 = {}, s3 = {}", s1, s3); // s1 and s3 are valid here



    // ---- references and borrowing ----
    // Rules of references 
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
    // 3. References are immutable by default.
    // 4. You can create a mutable reference using the &mut keyword.
    let s1 = String::from("hello"); 
    let len = calculate_length(&s1); // s1 is moved into the function and returned as part of the tuple
    println!("The length of '{}' is {}.", s1, len); // s2 is a reference to s1

}

// ---- ownerships and functions ----
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope but nothing special happens. The memory is not freed.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_string is returned and moved out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moved out to the calling function
}


// ---- references and borrowing ----
fn calculate_length(s: &String) -> usize {
    let length = s.len();   // len() returns the length of a String
    length // Return the String and its length as a tuple
}