use std::io;
use std::process;

fn main() {
    let a = 10;
    let b = 5;
    let c;

    let mut op = String::new();
 
    println!("Choose an operation");
    io::stdin().read_line(&mut op).expect("Failed");
    op = op.trim().to_string();
 
    if op == "+" {
        c = a + b;
    } else if op == "-" {
        c = a - b;
    } else if op == "*" {
        c = a * b;
    } else if op == "/" {
        c = a / b;
    } else {
        println!("Error!");
        process::exit(1);
    }
    println!("{}", c);
}
