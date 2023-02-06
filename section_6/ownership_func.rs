fn main() {
    let mut s = String::from("Hello");
    s = take(s);
    println!("{}", s);
}

fn take(s1: String) -> String {
    println!("{}", s1);
    s1
}
