use std::io;
 fn main() {
   let mut input=String::new();
   io::stdin().read_line(&mut input)
   .expect("Failed to read the line");

 
 for i in input.chars().rev(){
print!("{}",i);
 }

 }
