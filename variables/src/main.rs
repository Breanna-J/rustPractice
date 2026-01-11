fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    //rust variables are immutable, meaning it will not change. Using mut allows a variable value to change later.
    //const VARIABLE_NAME will not allow the value to be changed ever, useful for hardcoded values.
    x = 6;
    println!("The value of x is: {x}");

    let y = 6;

    //the second use of the let statement allows shadowing, meaning the value can change - the variable type
    let y = y + 4;

    //putting the variable statement in brackets allows it to be shadowed temporarily. Expiring afrer the second curly brace.
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is {y}");
}
