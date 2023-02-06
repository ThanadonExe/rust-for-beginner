fn main() {
    // Heap
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}", s2);

    // Stack
    let i1:i32 = 10;
    let i2 = i1;
    println!("{}", i2);
}
