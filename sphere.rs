const PI  : f32 = 3.14159265;
const DIV : f32 = 1.33333333;
use std::io;
fn main() {
    
    let mut radius = String::new();
    io::stdin().read_line(&mut radius);
    let radius : f32 = radius.trim().parse().unwrap();
    let volume : f32 = DIV*PI*radius*radius*radius; 
    println!("The Radius of Sphere is : {}",radius);
    println!("The Volume of Sphere is : {}",volume);
}
