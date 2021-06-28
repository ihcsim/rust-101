fn main() {
    println!("f(0) = {}", fibonacci(0));
    println!("f(1) = {}", fibonacci(1));
    println!("f(2) = {}", fibonacci(2));
    println!("f(3) = {}", fibonacci(3));
    println!("f(4) = {}", fibonacci(4));
    println!("f(5) = {}", fibonacci(5));
    println!("f(6) = {}", fibonacci(6));
    println!("f(7) = {}", fibonacci(7));
    println!("f(8) = {}", fibonacci(8));
}

fn fibonacci(x: i64) -> i64 {
    if x == 0 {
        return 0;
    }

    if x == 1 {
        return 1;
    }

    return fibonacci(x - 1) + fibonacci(x - 2);
}
