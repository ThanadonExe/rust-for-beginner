use std::io;

fn main() {
    let mut ans = String::new();

    for i in 1 .. 4 {
        ans.clear();
        println!("What's the capital city of Thailand");
        io::stdin().read_line(&mut ans).expect("Failed");
        ans = ans.trim().to_string().to_lowercase();
        if ans == "bangkok" {
            println!("Correct!");
            break;
        } else {
            if i < 3 {
                println!("Wrong! Try again.");
            } else {
                println!("You lose!");
            }
        }
    }
}
