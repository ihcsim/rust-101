use std::collections::HashMap;

static DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth",
    "eleventh", "twelfth",
];

const INDENT: &str = "    ";

fn main() {
    let mut lyrics = HashMap::new();
    lyrics.insert("first", "A partridge in a pear tree");
    lyrics.insert("second", "Two turtle doves");
    lyrics.insert("third", "Three French hens");
    lyrics.insert("fourth", "Four calling birds");
    lyrics.insert("fifth", "Five gold rings");
    lyrics.insert("sixth", "Six geese a laying");
    lyrics.insert("seventh", "Seven swans a swimming");
    lyrics.insert("eighth", "Eight maids a milking");
    lyrics.insert("nineth", "Nine ladies dancing");
    lyrics.insert("tenth", "Ten lords a leaping");
    lyrics.insert("eleventh", "Eleven pipers piping");
    lyrics.insert("twelfth", "Twelve drummers drumming");

    print_lyrics(0, lyrics);
}

fn print_lyrics(x: u32, lyrics: HashMap<&str, &str>) {
    if x >= DAYS.len() as u32 {
        return;
    }

    let day = DAYS[x as usize];
    let mut output = format!("On the {} day of Christmas my true love gave to me\n", day);
    output += INDENT;
    output += lyrics[day];

    for n in (0..x).rev() {
        if n > 0 {
            output += ", ";
        } else {
            output += " and ";
        }
        output += "\n";
        let last_day = DAYS[n as usize];
        output += INDENT;
        output += lyrics[last_day];
    }
    println!("{}", output);

    print_lyrics(x + 1, lyrics);
}
