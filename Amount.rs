use std::io;

fn main(){
  println!("Please enter principal amount:");
  let mut input_01 = String::new();
  io::stdin().read_line(&mut input_01);
  let input_01 : f32= input_01.trim().parse().unwrap();
  
  println!("Please Enter Rate of interest in %:");
  let mut input_02 = String::new();
  io::stdin().read_line(&mut input_02);
 let input_02 : f32 = input_02.trim().parse().unwrap();

  println!("Enter number of years for investment:");
  let mut input_03 = String::new();
  io::stdin().read_line(&mut input_03);
  let input_03 : f32 = input_03.trim().parse().unwrap();

  let total : f32 = input_01*(1.0+input_02)*input_03;
  println!("After {} years your principal amount {} over an interest rate of {} % will be {}",input_03,input_01,input_02,total);
  }
    

