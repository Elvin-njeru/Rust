use std::io;

fn main() {
    // Variable declarattion

    let x = 123;
    let y = 7;
    let name = "ELvin";

    // Print variables
    println!("value of X is {x}");
    println!("Value of Y is {y}"); 
    println!("My name is {name}");

    // INput collection
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline ");
    println!("User guessed {guess}");

    //compound types 
    // 1. tuple 
    let tup = (12.5,45,67);
    let value = tup.2;
    println!("value 2 in the tuple is {value}");
    
    // 2. array
    let array = [1,2,3,4,5,6,7,8,9];
    println!("Enter the index of the number you wan t aaccess in the array");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to readline");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let number = array[index];
    println!("The number is {number}");



}
