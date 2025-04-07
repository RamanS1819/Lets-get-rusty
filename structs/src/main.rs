// Structs
// Structs are used to create custom data types in Rust.
// They are similar to classes in other programming languages.
// Like tuple, structs can be used to group related data together of different types.
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

///// example usecase
#[derive(Debug)]    //-> here Debug is a trait that allows us to print the struct in a readable format
struct Rectangle {
    width: u32,
    height: u32,
}

// implement block has the functions and methods that are associated with the struct
impl Rectangle {
    // we can also define methods for the struct using impl
    // this is similar to classes in other programming languages
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // inside the impl block we can also define assosiated functions
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// structs allows us multiple implementation blocks
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user1 = User {
        //these can be in any order
        email: String::from("xyz123@gmail.com"),
        username: String::from("xyz123"),
        active: true,
        sign_in_count: 1
    };


    // we can get specific values from structs using dot notation
    let name = user1.username;
    // we can also modify the values of the struct using dot notation
    user1.username = String::from("xyz456");


    // creating another user with build_user function
    let user2 = build_user(
        String::from("abcd12@gmail.com"), 
        String::from("abcd12")
    );


    // we can also create a new instance of user using existing instances
    let user3 = User {
        email: String::from("raman123@gmail.com"),
        username: String::from("raman123"),
        ..user2
        // we can use the .. syntax to copy the values from user1
        // this is called struct update syntax
        // this will copy the values of sign_in_count and active from user1
    };


    // we can also create structs without name fields
    // these are called tuple structs
    // they are used to create a new type of struct with the same data types
    // this is useful when we want to create a new type of struct with the same data types
    struct Color(u32, u32, u32);

    struct Point(u32, u32, u32);



    ///// example usecase
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 20,
        height: 60
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);
    // we can also use the dot notation to get the values of the struct

    println!(
        "The area of the rectangle is {} square pixels.", 
        rect.area()
    );

}



// we can also use functions to construct the new instances of user.
// this function takes in the email and username and returns a new instance of user
// this is a common pattern in Rust to create new instances of structs
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}


// when using impl we don't need this function
/* 
///// example usecase
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
    */