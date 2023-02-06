fn main() {
    slice_string();
    slice_array();
}

fn slice_string() {
    let a = String::from("Hello_ World");
    let b = &a[0..=5];
    println!("{}", b); 
}

fn slice_array() {
    let a = [1, 2, 3, 4, 5];
    let b = &a[0..=2];
    println!("{:?}", b); 
}
