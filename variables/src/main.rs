use std::io;

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

    /*
     * Signed variant store numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive
     * Unsigned variant store numbers from 0 to 2^n - 1
     *
     * In a release version, if a number overflows, it will not cause a panic during runtime.
     * Instead, the number will wrap n % 255 if we use u8, which can cause undesired behaviours
     *
     * You can use methods attached to primitive numeric types:
     * - You can handle wrapping by using wrapping_* methods,
     * - return None if there is overflowing with the checked_* methods,
     * - return value and Boolean if there's an overflow with overflowing_* methods, or
     * - saturate the value's min and max values with saturating_* methods (limit any operations to
     * the max and min of that number)
     */

    // Array/tuple access is using . (e.g. let x: (i32, f64, u8) = (500, 6.4, 1); let five_hundred = x.0;)
    //
    // NOTE: You can declare array/tuple type as let a: **[i32; 5]** = **[1; 5]** = [1, 1, 1, 1, 1]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read index");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("Value at index {index} is {element}"); // Will raise panic if index is out of bounds
}
