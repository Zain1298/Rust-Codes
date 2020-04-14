

use std::io;
fn main() {
    println!("Enter a character:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input : char = input.trim().parse().unwrap();
    

if((input=='A' || input=='E' || input=='I' || input=='O' || input=='U' || input=='a' || input=='e' || input=='i' || input=='o' || input=='u' )) {
  println!("Letter {} is Vowel",input);
} else {
  println!("This Letter is not a Vowel");
}
}
