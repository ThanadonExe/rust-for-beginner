fn main() {
    let a:(i32, bool, f64) = (5, true, 12.8);
    println!("{:?}", a);
    print_tup(a);
}

fn print_tup(b:(i32, bool, f64)) {
    let (x, y, z) = b;
    println!("{} {} {}", x, y, z);
}
