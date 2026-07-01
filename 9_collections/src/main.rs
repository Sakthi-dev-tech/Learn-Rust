#[allow(unused)]
fn main() {
    // --------------------- VECTOR ------------------------------

    /* A vector allows you to store more than one value in a single data structure that puts all
     * values next to each other in memory.
     * Can only store same type of value
     * E.g. lines of text in a file, or prices of item in a shopping cart
     */

    // Vector is dynamically sized compared to an array
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Can add elements to a mutable vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // Reading from vector
    let third: &i32 = &v[2]; // this will cause a panic in runtime if index doesn't exist

    // If we are not sure about an element at that index
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    /* However trying to read the first element, then pushing another element and trying to print
     * the first element will cause a compile error as both mutable and immutable reference to the
     * vectors are being used
     *
     * This is because vectors put values next to each other in memory, adding a new element onto
     * the end of the vector might require allocating new memory and copying the old elements to the
     * new space, if there isn't enough room to put all the elements next to each other where the
     * vector is currently stored. Which could cause reference to a deallocated memory if a
     * reference is called after a mutable reference.
     */

    // Can iterate through a vector
    for i in &mut v {
        // * dereferences the operator to get to the value in i before using the += operator
        *i += 50;
    }

    // Enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // --------------------------------- STRING ---------------------------------------

    // to_string method is available on any type that implements Display trait
    // Same as doing String::from
    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    // Strings are UTF-8 coded so you can include any peroply encoded data
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Concaatenating Strings

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // implicily uses add method
    // fn add(self, s: &str) -> String
    let s3 = s1 + &s2;

    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Use format macro to have a nicely formatted string code
    let s = format!("{s1}-{s2}-{s3}");

    /* Although this is a vector and can be indexed, it is not allowed as different charcters would
     * require different number of bytes and the indexing could be off
     */
    // Instead, we could convert it to char or bytes as required
    for c in "Зд".chars() {
        println!("{c}");
    }

    // -------------------------------------------- HASHMAP ---------------------------

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let team_name = String::from("Blue");
    // copied will return a Option<i32> from a Option<&i32> then unwrap_or will get the score or
    // return 0 by default
    let score = scores.get(&team_name).copied().unwrap_or(0);
    scores.entry(String::from("Blue")).or_insert(50); // insert Blue -> 50 iff Blue does not exist

    for (key, value) in &scores {
        println!("{key} scored {value}")
    }


    // For types that implement Copy trait, like i32, values are copied into the hashmap.
    // For owned values like String, values will be moved and the hashmap will be the owner of those
    // values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    
    // Making a counter of words
    let text = "hello wonderful world!";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
