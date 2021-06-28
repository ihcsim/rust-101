fn main() {
    println!("Hello, world!");
    another_function(1, 2);
    println!("incrementing 2: {}", inc(2));
}

fn another_function(x: i32, y: i32) {
    println!("call another function with {} and {}", x, y);
}

fn inc(x: i32) -> i32 {
    x + 1
}
