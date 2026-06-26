#![allow(unused)]

use std::net::IpAddr;

fn main() {
    // enum states that this value is one of a possible set of values
    enum IpAddrKind {
        V4,
        V6,
    }

    // create instances of each of the two variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    struct IpAddrStruct {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // The same as above but including the address within the enum
    // without adding a struct
    enum IpAddrEnum {
        V4(String),
        V6(String),
    }

    let home_enum = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback_enum = IpAddrEnum::V6(String::from("::1"));

    // We can also have different types stored within the enum
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Sample code to show match in action
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alaska => year >= 1959,
                UsState::Alabama => year >= 1819,
            }
        }
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    // Another way of doing match
    fn describe_state_quarter(coin: Coin) -> Option<String> {
        if let Coin::Quarter(state) = coin {
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old, for America!"))
            } else {
                Some(format!("{state:?} is relatively new!"))
            }
        } else {
            None
        }
    }

    // Same way of doing the above function
    fn describe_state_quarter_let_else(coin: Coin) -> Option<String> {
        // Happy state: If coin was created as Coin::Quarter(UsState::<state>), the pattern matches
        // and assignes it to a new variable called state
        //
        // Sad Path: If coin was anything else, it doesn't match and jumps to the else block
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    // Option is like Optional in Java
    // match here needs to cover all the cases of enum for compiler to not complain
    // you can use _/other to mention default case when no other cases match
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
