use std::io;
    
fn main() {
    println!("Enter a Binary number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut input:i32 = input.trim().parse().unwrap();

    let mut rem = 0;
    let mut sum = 0;
    let mut total = 1;

while input != 0{

rem = input % 10;
sum = sum + rem*total;
input = input / 10;
total = total*2;

}
    println!("Decimal Representation of entered binary number is {}",sum );

}




 