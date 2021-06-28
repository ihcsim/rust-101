fn main() {
    let x = 32.0;
    println!("{}C = {}F", x, to_fahrenheit(x));

    let y = 81.0;
    println!("{}F = {}C", y, to_celsius(y));
}

fn to_fahrenheit(x: f32) -> f32 {
    x * 9.0 / 5.0 + 32.0
}

fn to_celsius(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}
