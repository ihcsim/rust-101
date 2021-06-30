fn main() {
    let s = "apple".to_string();
    let converted = pig_latin(&s);
    println!("{} => {}", s, converted);

    let s = "eat".to_string();
    let converted = pig_latin(&s);
    println!("{} => {}", s, converted);

    let s = "are".to_string();
    let converted = pig_latin(&s);
    println!("{} => {}", s, converted);

    let s = "first".to_string();
    let converted = pig_latin(&s);
    println!("{} => {}", s, converted);

    let s = "pig".to_string();
    let converted = pig_latin(&s);
    println!("{} => {}", s, converted);

    let s = "banana".to_string();
    let converted = pig_latin(&s);
    println!("{} => {}", s, converted);

    let s = "qwbanana".to_string();
    let converted = pig_latin(&s);
    println!("{} => {}", s, converted);
}

fn pig_latin(s: &String) -> String {
    let mut index = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                if i == 0 {
                    let mut clone = s.clone();
                    clone.push_str("-hay");
                    return clone;
                }

                index = i;
                break;
            }

            _ => {
                continue;
            }
        }
    }

    let front = s[index..].to_string();
    let back = s[0..index].to_string();
    format!("{}-{}ay", front, back)
}
