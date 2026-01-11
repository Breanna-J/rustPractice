use std::io;
use std:cmp::Ordering //standard comparison ordering
use rand::Rng; //random number generator

fn main() {
    
    //println! means that println is a macro, rather than a function, 
    // and is used to print to print to the standard output
    println!("Guess a number!");

    // to use random number generators, you need to update dependencies in the toml file
    // dependencies tell cargo the crates that your project depends. This one needs rand. add it to the top 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //{} is a placeholder that calls the variable inside 
    println!("The secret number is: {secret_number}");

    println!("Please input your guess:");

    //compare guess to the random number
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println("Too High!"),
        Ordering::Equal => println!("You Win!")
    }

    //by default, rust variables are not mutable (you cant change them when they are initalized)
    // the mut keyword allows the variable value to change
    let mut guess = String::new(); 

    io::stdin() //read the standard input from the io(input/output) module
        .read_line(&mut guess) //read line and replace the value of guess -- &mut guess references the variable
        .expect("Failed to read line."); //if the answer is not a string, as expected, print this. with no expect, you will get a warning for incorect input

    println!("You guessed: {guess}"); 

    


    
}
