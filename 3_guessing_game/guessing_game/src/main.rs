extern crate rand; // External Library

use std::io; // Standart Input Output Library
use rand::Rng; // Rand library
use std::cmp::Ordering;

fn main() { //Main Function
    println!("Welcome Guess the Number Application !"); // Output Terminal

    let secret_number = rand::thread_rng().gen_range(1, 101); // Random Count

    println!("The secret number is: {}", secret_number); // Print Random Value

 loop {
    println!("Please input your guess."); // Output Terminal

    let mut guess = String::new(); // create new string type mutable
    /*
    Mutable
        let foo = bar;
        Example :
            let test=5; // test  is immutable.
            let mut test =5; // test  is mutable.
    */

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        // io::stdin() is standard input for your terminal
        // readline : read and set mutable.
        // & is mutable adress.
        // .expect is if get error, read   "Failed to read line"


        let guess: u32 = match guess.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,
           };

    println!("You guessed: {} ", guess); // output, entered value

    match guess.cmp(&secret_number) {
              Ordering::Less    => println!("Too small!"),
              Ordering::Greater => println!("Too big!"),
              Ordering::Equal   => {
                  println!("You win!");
                  break;
              }
          }

    /*
    let x = 5;
    let y = 10;

    println!("x and y: {} and {}", x, y);
    */
}
}
