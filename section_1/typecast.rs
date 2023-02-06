fn main() {
    let a:i32 = 20;
    let b:i64 = a.into();
    let c:i64 = a as i64 + 10;

    println!("a {}", a);
    println!("b {}", b);
    println!("c {}", c);
}
