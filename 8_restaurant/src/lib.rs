mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetiser {
        Soup,
        Salad
    }

    fn fix_incorrect_order() {
        cook_order();
        // Super is like doing ../
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // This is the relative path version of the absolute path:
    // crate::front_of_house::hosting::add_to_waitlist();
    // Start an absolute path with crate
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // We then change our mind about what break we want
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // However, the following line will not run
    // meal.seasonal_fruit = String::from("blueberries");
    //
    // This is because we're not allowed to see nor modify it due to it not having the pub modifier

    // Because we made the enum public, we are able to use the Soup and Salad variants in
    // eat_at_restaurant
    let order1 = back_of_house::Appetiser::Soup;
    let order2 = back_of_house::Appetiser::Salad;
}

fn deliver_order() {}
