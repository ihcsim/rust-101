fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("highest: {}", highest(&v));

    let v = vec![5, 4, 3, 2, 1];
    println!("highest: {}", highest(&v));

    let v = vec![5];
    println!("highest: {}", highest(&v));

    let v = vec![4, 5, 2, 1, 3];
    println!("highest: {}", highest(&v));
}

fn highest(v: &[i32]) -> i32 {
    let mut highest = v[0];
    for &i in v {
        if i > highest {
            highest = i;
        }
    }

    highest
}
