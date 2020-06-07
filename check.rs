
//Q2)Write a Rust program to check if a number is positive, negative or zero.

use std::io;
fn main() {
    println!("Enter Any Number");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input : i32 = input.trim().parse().unwrap();
    
    if input == 0 {
   println!("Zero Entered");
  } else if input > 0 {
     println!("Positive Number Entered");
   } else {
     println!("Negative Number Entered");
   }
}