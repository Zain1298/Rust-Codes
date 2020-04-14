use std::io;
fn main() {
    println!("Enter Height in Cm:");

    let mut height = String::new();
    
    io::stdin().read_line(&mut height);
    
    let height : f32= height.trim().parse().unwrap();
    
    let converter : f32 = height / 100.0;
    
    println!("Enter Weight in :");
    
    let mut weight = String::new();
    
    io::stdin().read_line(&mut weight);
    
    let weight : f32 = weight.trim().parse().unwrap();
    
    let height_square = converter * converter;
    
    let bmi = weight/height_square;
    
    println!("Your BMI is {}",bmi);
 
}
