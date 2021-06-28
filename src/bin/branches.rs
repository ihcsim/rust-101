fn main() {
    let x = 100;
    if x % 5 == 0 {
        println!("x is divisible by 5");
    } else if x % 4 == 0 {
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 {
        println!("x is divisible by 2");
    } else {
        println!("x is not divisible by 5, 4, 3 or 2");
    }

    let condition = true;
    let y = if condition { 4 } else { 5 };
    println!("y is {}", y);

    let mut index = 0;
    loop {
        if index > 10 {
            break;
        }

        println!("hello {}", index);
        index += 1;
    }

    let mut count_down = 3;
    while count_down > 0 {
        count_down -= 1;
        println!("not ready");
    }
    println!("finished!");

    let arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for elem in arr.iter() {
        println!("{}", elem);
    }
}
