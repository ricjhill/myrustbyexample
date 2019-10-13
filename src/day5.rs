// mod is used on main.rs to pull in this file as a module
// https://stackoverflow.com/questions/26388861/how-to-include-module-from-another-file-from-the-same-project
//
// Contents of main.rs
/*
mod day5;

fn main() {
    day5::print_hello();
}

*/
    pub fn print_hello() {
        println!("Hello, world!");
    }
