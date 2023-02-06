use std::io;

fn main() {
    let mut c = String::new();
    println!("Enter a character");
    io::stdin().read_line(&mut c).expect("Failed");
    let c:char = c.trim().parse().expect("Failed");

    if c == "a" || c == "e" || c == "i" || c == "o" || c == "u" {
        println!("Vowel!");
    } else {
        println!("Not Vowel!");
    }
}
