use std::collections::HashMap;

fn main() {
    let vec = vec![1, 5, 10, 2, 15, 1];
    let (mean, median, mode) = calc_all(&vec);
    println!("mean: {}\nmedian: {}\nmode: {}", mean, median, mode);
}

fn calc_all(v: &Vec<i64>) -> (i64, i64, i64) {
    let mut clone = v.clone();
    clone.sort();

    let num_elem = clone.len();
    let median: i64 = clone[num_elem / 2];

    let sum: i64 = clone.iter().sum();
    let mean = sum / num_elem as i64;

    let mut counts: HashMap<i64, i64> = HashMap::new();
    for i in &clone {
        *counts.entry(*i).or_insert(1) += 1;
    }
    let mut highest: i64 = 0;
    let mut mode: i64 = 0;
    for (num, count) in counts {
        if count > highest {
            highest = count;
            mode = num;
        }
    }

    (mean, median, mode)
}
