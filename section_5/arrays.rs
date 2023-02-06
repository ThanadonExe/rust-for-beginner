fn main() {
    let mut a:[i32;5] = [3;5];
    a[3] = 1;
    println!("{:?}", a);

    print(a);
    print_iter(a);
}

fn print(x:[i32;5]) {
    for n in 0 .. 5 {
        println!("{}", x[n]);
    }
}

fn print_iter(x:[i32;5]) {
    println!("Length : {}", x.len());
    for n in x.iter() {
        println!("{}", n);
    }
}