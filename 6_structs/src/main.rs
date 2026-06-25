fn main() {
    // Define a struct using
    #[allow(unused)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // we can access the fields using user.FIELD_NAME
    let user = User {
        active: true,
        username: String::from("username123"),
        email: String::from("user@gmail.com"),
        sign_in_count: 1,
    };

    let mut mut_user = User {
        active: user.active,
        username: String::from("username123"),
        email: String::from("user@gmail.com"),
        sign_in_count: 1,
    };

    mut_user.email = String::from("new@gmail.com");

    // We can skip the repetition of rewriting user by creating a function
    #[allow(unused)]
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1
        }
    }

    /*
     * Since this is an assignment, we can no longer use user as the data is moved into user2
     * However, if we had given new String values for both email and username, and only used active
     * and sign_in_count, user will still be valid as their type implements the copy method
     */
    #[allow(unused)]
    let user2 = User {
        email: String::from("another@example.com"),
        ..user
    };

    #[derive(Debug)] // this is so that we can call dbg! on this
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    // implement a method to the struct
    // methods can take Ownership of self, borrow self immutably, or borrow self mutably
    impl Rectangle {
        //  &self is short for self: &self
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // Like Java comparison method
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // Return a new instance of the struct
        #[allow(unused)]
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(scale * 30), // dbg! returns an owenership and hence can be used here
        height: 50
    };

    let rect2 = Rectangle{
        width: 200,
        height: 100
    };

    println!("Area of rectangle is {} square pixels", rect1.area());                
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    dbg!(rect1); // dbg! macro allows us to print formatted debug message compared to doing
                 // println!("rect1 is {rect1:?}")
}
