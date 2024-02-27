// use 'use' to bring something into the scope (like '#include'. is it a macro processed by
// proprocessor?)
use std::io;

fn main(){
    println!("=====Guess the number====="); // println! has been included in the prelude
    println!("Please input your guess");
    
    //let apples = 5;
    //let mut banana = 114;
    let mut guess = String::new(); // String is growable in Rust!
    
    // std::io::stdin if not use std::io in line 3
    io::stdin()
        .read_line(&mut guess) // mutable reference; read_line() return a result value for excepion
                               // handling
        .expect("Failed to read line!"); // except is a method of Result type

    println!("You guessed: {guess}");
}
