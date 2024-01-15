// fn main(){
//     // println!("hello world")
//     let mut apples=5;
//     apples=7;
//     println!("{apples}");
// }

// guessing game 

use std::io;
fn main(){
    println!("Guess the number !");
    println!("Please input your guess.");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line.");
    println!("You guessed: {guess}");
}
