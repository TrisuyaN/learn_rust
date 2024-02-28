// use 'use' to bring something into the scope (like '#include'. is it a macro processed by
// proprocessor?)
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("=====Guess the number====="); // println! has been included in the prelude
 
    let secret_num = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is {secret_num}");

    loop{
        println!("Please input your guess");
    
        //let apples = 5;
        //let mut banana = 114;
        let mut guess = String::new(); // String is growable in Rust!
    
        // std::io::stdin if not use std::io in line 3
        io::stdin()
            .read_line(&mut guess) // mutable reference; read_line() return a result value for excepion
                                   // handling
            .expect("Failed to read line!"); // except is a method of Result type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
