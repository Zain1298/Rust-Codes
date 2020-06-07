use std::io;
fn main() {
    
    println!("Enter Height in Feet:");

    let mut height = String::new();
    io::stdin().read_line(&mut height);
    let height : f32= height.trim().parse().unwrap();
    let converter : f32 = height * 30.48 ;
    println!("There are {} Cm in {} ft",converter,height);
    
 }
