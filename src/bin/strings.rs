fn main() {
    let mut x = String::from("hello");
    x.push_str(", world!");
    println!("{}", x);

    // move variable
    let y = x;
    println!("y = '{}'", y);

    // clone variable as mutable
    let mut z = y.clone();
    z.push_str(" again!");
    println!("y = '{}'", y);
    println!("z = '{}'", z);

    let a = String::from("give me another!");
    let mut b = a.clone();
    b.push_str("i'm a clone");
    println!("a = '{}'\nb = '{}'", a, b);

    // move variable into function
    let j = String::from("molly");
    print(j);

    // pass variable reference into function
    let k = String::from("holly");
    print_ref(&k);
    println!("k = {}", k);

    let word = String::from("hello world !");
    println!("first_word(\"hello world !\"); -> {}", first_word(&word));
}

fn print(s: String) {
    println!("s = {}", s)
}

fn print_ref(s: &String) {
    println!("s = {}", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
