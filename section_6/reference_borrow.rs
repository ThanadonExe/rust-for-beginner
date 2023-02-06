fn main() {
    let r = String::from("Hello");
    print_ref(&r);
    
    let mut b = String::from("Hello");
    print_borrow(&mut b);
    println!("after: {}", b);
}

fn print_ref(r1: &String) {
    println!("{}", r1);
}

fn print_borrow(b1: &mut String) {
    println!("before: {}", b1);
    b1.push_str("World");
}
