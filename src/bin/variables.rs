fn main() {
    // immutable vs mutable
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);

    // shadowing
    let y = 3;
    let y = y + 1;
    let y = y + 2;
    println!("the value of y is {}", y);

    // constant
    const MAX_POINTS: u32 = 100000;
    println!("the maximum points are {}", MAX_POINTS);

    // annotate variables with types
    let guess: u32 = "42".parse().expect("not a number");
    println!("guess is {}", guess);

    let a = 1;
    let b: i32 = 2;
    let c: u32 = 3;
    let d = 2.0;
    let e: f32 = 5.00;
    println!("see all the values: {} {} {} {} {}", a, b, c, d, e);

    // tuples
    let tup: (i32, bool, u8) = (-1, true, 1);
    println!("tup is {:#?}", tup);
    let (x, y, z) = tup;
    println!("destructure tup {} {} {}", x, y, z);

    // array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array is {:#?}", arr);
}
