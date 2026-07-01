use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
           ErrorKind::NotFound => match File::create("hello.txt") {
               Ok(fc) => fc,
               Err(error) => panic!("Problem creating file: {error}")
           },
           _ => {
               panic!("Problem opening the file: {error:?}")
           }
        },
    };

    let greeting_file_result = File::open("hello.txt")
        .expect("No hello.txt found!");

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(error) => return Err(error) // We do not handle the error here
        };

        let mut username = String::new();

        // We let this take care of the error from above
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file_alternative() -> Result<String, io::Error> {
        // ? operator called will go through the from function, defined in the From trait in the std
        // library, that is used to convert one type into another. Hence, the error type received is
        // converted into the error type defined in the return type of the current function. 
        //
        // Useful when a function returns one error type to represent all the ways a function might
        // fail
        //
        // For any custom error type, we need to define impl From<io::Error>
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
}
