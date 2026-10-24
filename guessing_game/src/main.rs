use std::io;   // brings input out library from standard files ig
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");

    let secret_number=rand::random_range(1..=100);
    println!("The secret number is {secret_number}");


    println!("please input your guess.");

    let mut guess=String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    println!("you guessed: {guess}");   
    // let x =10;
    // let y=20;
    // println!("x ={x}\ny+2={}",y+2);



}
