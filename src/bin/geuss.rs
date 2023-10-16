use std::io;

fn main(){
    println!("Enter a number");
    let mut geuss = String::new();
    io::stdin().read_line(&mut geuss).expect("Failed to read line");
    println!("THe number you geussed is {}",geuss)
}