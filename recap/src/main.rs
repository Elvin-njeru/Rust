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

    //FUNCTIONS
    println!("Enter the first number");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Fialed to read line");
    let num1:i32 = num1.trim().parse().expect("Number 1 was not a number");

    println!("Enter the second number");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("failed to readline");
    let num2:i32 = num2.trim().parse().expect("The number entered is not an integer");

    addition(num1,num2);

    // return types
    let x = 2;
    plus_one(x);

    //if conditions
    let mut z = 1;
    loop {
        if z > 3 {
            println!("The value is at max");
            break;
        }else {
            z = z + 1 ;
            println!("Value not max yet");
        }
    }


}

fn addition(num1:i32, num2:i32){
    let answer = num1 + num2;
    println!("The sum of the 2 numbers is {answer}");
} 

// function with a return type
fn plus_one(x:i32) -> i32 {
    x+1
}