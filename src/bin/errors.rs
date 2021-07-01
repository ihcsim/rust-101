use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    let f1 = File::open("hello.txt");
    let f1 = match f1 {
        Ok(file) => file,
        Err(error) => panic!("can't open file: {:?}", error),
    };

    let f2 = File::open("hello.txt");
    let f2 = match f2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("failed to create file: {}", e),
            },
            other_error => {
                panic!("can't open file: {:?}", other_error)
            }
        },
    };

    let f3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("can't create file: {:?}", error);
            })
        } else {
            panic!("can't openf ile: {:?}", error);
        }
    });

    let f4 = File::open("hello.txt").unwrap();
    let f5 = File::open("hello.txt").expect("can't open file");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
