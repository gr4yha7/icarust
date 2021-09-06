use std::io;

fn main() {
  println!("Enter your age:");
  let mut age = String::new();
  io::stdin().read_line(&mut age).expect("failed to read line");
  let age: usize = age.trim().parse().expect("age is not a number");
  println!("Age = 0x{:x}", age)
}