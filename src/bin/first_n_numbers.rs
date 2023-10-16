use std::io;

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n:u32 = n.trim().parse().expect("Failed to parse and trim");
    println!("First {} natural numbers are",n);
    for i in 1..n{
        println!("{}" ,i);
    }
}