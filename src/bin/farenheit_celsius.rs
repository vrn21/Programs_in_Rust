use std::io;

fn main() {
  let mut f = String::new();
  println!("Enter the temperature in farenheit");
  io::stdin().read_line(&mut f).expect("Failed to read line");
  let f:i32 = f.trim().parse().expect("Error");
  println!("The converted value in degree is {}", (f - 32) * 5 / 9);
}