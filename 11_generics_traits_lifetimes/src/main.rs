#[allow(unused)]
fn main() {
    // ---------------------------- GENERICS -------------------------------
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    // --------------------------- TRAITS -----------------------------------------

    // Any type that has teh Summary trait will have the method summarise defined with the method signature
    // exactly
    pub trait Summary {
        fn summarise(&self) -> String;
        fn summarise_default(&self) -> String {
            // this is to implement a default behaviour
            String::from("Read more...")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarise(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    impl Summary for SocialPost {
        fn summarise(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarise());
    println!("Default behaviour: {}", post.summarise_default());

    // Take in any type that implements Summary
    // We can also concatenate implementation like
    // &(impl Summary + Display)
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarise());
    }

    // A more verbose alternative
    pub fn notify_alternative<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarise());
    }

    /*
     * We can write as following if we have a lot of traits to concatenate by using where
     * fn some_function<T, U>(t: &T, u: &U) -> i32
     * where
     *      T: Display + Clone,
     *      U: Clone + Debug,
     *   {
     */

    /* Std libraries implement the ToString trait on any type that implement the Display trait by
     * impl<T: Display> ToString for T {
     *
     * }
     *
     */

    // --------------------------------- LIFETIMES -----------------------------------------------

    // Lifetime of r is annotated with 'a and lifetime of x is annotated with 'b
    // Compiler compares the size of both lifetimes and see that r has a lifetime of 'a but refers
    // to memory with lifetime of 'b, thus throws compile error
    //
    // fn main() {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {r}");   //          |
    // }                         // ---------+

    /*
     * fn longest(x: &str, y: &str) -> &str {
     *      if x.len() > y.len() { x } else { y }
     *   }
     *
     *   This wont' compile as the compiler is unsure which lifetime (x or y) relate to the lifetime
     *   of the return value - even we don't know
     */

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    /*
     * The following function signature tells the compiler that for some lifetime 'a, the function
     * takes 2 params, both of which are string slices that live at least as long as lifetime 'a
     *
     * AKA lifetime of the reference returned by the longest function is the same as the smaller of
     * the lifetimes of the values referred by the function arguments
     *
     * This is not changing lifetime, but rejecting params that do not adhere to this constraint
     */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("long string is long");

    {
        // Compiler approves as string1 is valid until the end of the outer scope, string2 is valid
        // until the end of the inner scope and result references something that is valid until the
        // end of the inner scope
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // This means that an instance of ImportantExcerpt cannot outlive the reference it holds in its
    // part field
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // Implementing methods on a struct that has lifetimes
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    //  You don't always have to write lifetime annotations. The Rust compiler has three "Lifetime Elision Rules" programmed into it.
    //  If your code fits these rules, the compiler infers the lifetimes automatically:
    //
    // Rule 1: Each parameter that is a reference gets its own lifetime parameter.
    //
    // Rule 2: If there is exactly one input lifetime, that lifetime is assigned to all output lifetimes. (e.g., fn foo(x: &i32) -> &i32
    // becomes fn foo<'a>(x: &'a i32) -> &'a i32).
    //
    // Rule 3: If it's a method and one of the parameters is &self or &mut self, the lifetime of self is assigned to all output lifetimes.

    /*
     * There is a special lifetime called 'static, which denotes that the reference can live for the
     * entire duration of the program.
     */
}
