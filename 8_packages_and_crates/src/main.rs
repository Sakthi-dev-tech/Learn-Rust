// Declaring Modules
// compiler will look for this module's code in 3 places
// - Inline, within curly brackets that replace the semicolon following mod garden
// - In the file src/garden.rs 
// - In the file src/garden/mod.rs
pub mod garden; // tells the compiler to include the code found in src/garden.rs

use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus{};
    println!("I'm growing {plant:?}!")
}
