//make a main function that calls the add function in lib and prints the result
//import add from lib

use steam_cli::add;

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add(1, 2));
}
