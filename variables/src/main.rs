fn main() {
    // const are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    let x = 5;
    let x = x + 1; // this variable is said to shadow over the previous x declaration
                   // NOTE: This is a new declaration unlike when using mut
    
    // shadowing is useful when we want a certain variable type but it is derived from a different
    // type. For e.g.
    let spaces = "   ";
    let spaces = spaces.len(); // when I just want to know the length of spaces
    // if I use mut here when declaring (let mut), I get an error because when I use mut, I am
    // reusing the same variable, and the type of the variable cannot be changed
    
    {
        let x = x * 2;
        println!("Value of x in the inner loop is {x}"); // will be 12 here as it uses the x from the outer scope
                                                         // but this new x will be within this scope
    }

    println!("The value of x is {x}"); // value of x will not be affected by the inner loop and
                                       // thus, remains as 6
}
