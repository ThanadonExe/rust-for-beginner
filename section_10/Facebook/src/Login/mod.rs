pub fn login(username: String, password: String) -> bool {
    if username == "adam" && password == "password" {
        println!("Logged in!");
        true
    } else {
        false
    }
}
