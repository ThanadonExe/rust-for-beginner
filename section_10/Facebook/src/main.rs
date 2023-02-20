extern crate Facebook;
use std::io;
use Facebook::*;

fn main() {
    let mut username = String::new();
    let mut password = String::new();
    let mut post = String::new();

    println!("Enter your username: ");
    io::stdin().read_line(&mut username).expect("Failed");
    username = username.trim().to_string();

    println!("Enter your password: ");
    io::stdin().read_line(&mut password).expect("Failed");
    password = password.trim().to_string();
    
    if Login::login(username, password) {
        println!("Enter your post:");
        io::stdin().read_line(&mut post).expect("Failed");
        post = post.trim().to_string();
        Post::post(post);
        Logout::logout();
    } else {
        println!("Invalid Username or Password");
    }
}
