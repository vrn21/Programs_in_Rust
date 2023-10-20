use std::io;

fn factorial(n:i32) -> i32{
    if n == 1{1}
    else{n*factorial(n-1)}
}

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Invalid");
    let n:i32 = n.trim().parse().expect("Error");
    println!("the factorial is {}",factorial(n));
}
