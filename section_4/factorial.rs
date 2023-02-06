fn main() {
    let n = 5;
    let f = factorial(n);
    println!("Factorial of {} is {}", n, f);
}

fn factorial(n:i32) -> i32 {
    let mut f:i32 = 1;
    for i in 1 .. n+1 {
        f = f * i;
    }
    f
}
