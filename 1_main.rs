use std::io;
fn main() {
    println!("Enter a Decimal number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut input:i32 = input.trim().parse().unwrap();


let mut rem = 0;
let mut sum = 0;
let mut total = 0;
let mut i = 1;

sum=input;
while input !=0{
  rem = input % 2;
  input = input / 2;
  total = total + (rem * i);
  i=i*10;
}
println!("Binary Representation of {} is {}",sum,total);
}
