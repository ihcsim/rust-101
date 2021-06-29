fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    v1.push(9);

    let third: &i32 = &v1[2];
    println!("the third element v1 is {}", third);

    let v2 = vec![1, 2, 3];
    match v2.get(2) {
        Some(third) => println!("the third element of v2 is {}", third),
        None => println!("v2 has no third element"),
    }

    println!("before mut:");
    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v1 {
        *i += 60;
    }

    println!("after mut:");
    for i in &v1 {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row)
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
