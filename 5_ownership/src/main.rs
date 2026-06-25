fn main() {
    /*
     * Summary of stack and heap
     * Stack:
     *      - Stores value in order (LIFO)
     *      - Add data => push onto stack
     *      - Remove data => Pop stack
     *      - Data stored on the stack must have a known, fixed size
     *      - Data with an unknown size or compile time or size must be stored on heap
     *
     *  Heap:
     *      - When putting data on heap, request a certain amt of space.
     *      - Mem alloc finds empty spot in heap that is big enough => marks as used => returns a
     *          pointer
     *      - Pointer to heap is known and fixed => can be stored on stack
     *      - Allocating space on heap requires more effort than on stack as allocator needs to find
     *          a big enough space to hold the data => perform bookkeeping to prepare for next alloc
     *
     *  Usually the CPU will take a chunk of neighbouring data when fetching data from memory to
     *  store in CPU cache for faster access, as it assumes if you need data from point A, you will
     *  probably need data near it. However, this logic fails in the heap as the heap is
     *  disorganised, and that neighbouring data is usually not related to the current task.
     *
     *  Ownership Rules of Rust
     *  1. Each value in Rust has an owner
     *  2. There can only be one owner at a time
     *  3. When the owner goes out of scope, the value will be dropped
     */

    let s1 = String::from("hello");
    #[allow(unused)]
    let s2 = s1;

    // println!("{s1}, world!");

    /*
     *  In other languages, this would be a shallow copy where s1 and s2 refer to the same object.
     *  However, in Rust, it will be a move where s1 will no longer be valid (s1 was moved into s2)
     *  This helps to prevent both s1 and s2 trying to free the memory when they go out of scope
     */

    #[allow(unused)]
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // To perform deep copy
    
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1 = {s1} s2 = {s2}");

    // However, when we use primitives like integers, we know its size at compile time and thus,
    // stored entirely on the stack => copies of the actual values are quick to make => no need to
    // call clone here
    let x = 5;
    let y = x;

    println!("x = {x} y = {y}");

    // Another example of ownership in function
    let s = String::from("hello"); // s in scope

    takes_ownership(s); // s's value move into this function
                        
    // calling s now will cause compile time error as it has alr been dropped

    fn takes_ownership(str: String) {
        println!("{str}");
    } // s goes out of scope and "drop" is called. The backing memory is freed
      
    /*
     * You are unable to have both mutable and immutable reference in the same scope
     */

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s; // no problem as the scope for the immutable references (r1 and r2) end after
                     // its print statement, before the mutable reference is created. Hence, these
                     // scopes don't overlap and it is allowed
    println!("{r3}");
}
