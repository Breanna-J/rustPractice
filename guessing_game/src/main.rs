use std::io;

fn main() {
    println!("Guess a number!");//println! means that println is a macro, rather than a function, 
    // and is used to print to print to the standard output

    println!("Please input your guess:");

    //by default, rust variables are not mutable (you cant change them when they are initalized)
    // the mut keyword allows the variable value to change
    let mut guess = String::new(); 

    io::stdin() //read the standard input from the io(input/output) module
        .read_line(&mut guess) //read line and replace the value of guess -- &mut guess references the variable
        .expect("Failed to read line."); //if the answer is not a string, as expected, print this. with no expect, you will get a warning for incorect input

    println!("You guessed: {guess}"); //{} is a placeholder that calls the variable inside 

    // to use random number generators, you need to update dependencies in the toml file
    // dependencies tell cargo the crates that your project depends. This one needs rand.

    
    
}
