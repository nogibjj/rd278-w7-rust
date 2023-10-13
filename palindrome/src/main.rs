mod lib;

use lib::check_palindrome;
use std::io;

fn main() {
    // Read input from the terminal
    let mut input = String::new();
    println!("Enter a word:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Remove newline character from input
    let word = input.trim();

    // Check if the word is a palindrome
    let result = check_palindrome(word);

    // Print the result to the terminal
    println!("{}", result);
}
