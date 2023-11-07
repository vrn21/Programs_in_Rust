use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guessing Game !!!");
    print!("Enter quit to quit the game:");
        
    let secret_number = rand::thread_rng().gen_range(1..=101);
    loop {
       println!("Enter your Number geuss, number is between 1 and 100");
       let mut geuss = String::new();
        io::stdin().read_line(&mut geuss).expect("Failed to read line");

        let geuss: u32 = match geuss.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
        println!("you geussed: {}", geuss);

        match geuss.cmp(&secret_number){
            Ordering::Less => println!("The number input is smaller, than the secret number"),
            Ordering::Greater => println!("The number input is greater, than the secret number"),
            Ordering::Equal => {
                println!("Yay! You geussed it right, You won!!");
                break;
            }
        }
    }
    
    
}