use std::io;

fn main(){
    println!("ENter number upto which you want fibonacci series");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("error");
    let n:u32 = n.trim().parse().expect("error");
    let mut a = 0 ;
    let mut b = 0;
    let mut c = 1 ;
    print!("Fibonacci series -> {} {}",a,b);
    for i in 1..n{
        print!(" {}",c);
        a = b;
        b = c;
        c = a +b;
    }
 
}