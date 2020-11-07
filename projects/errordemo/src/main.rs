use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    // read_one();
    // read_two();
    read_username_from_file2().expect("not fount hello.txt file!");
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // let mut s = String::new();
    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;
    //// File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    fs::read_to_string("hello.txt")
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_one() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("create file error: {:?}", e),
            },
            other_error => panic!("other error {:?}", other_error),
        },
    };
}

fn read_two() {
    let _f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("error {:?}", error);
            });
        } else {
            panic!("error: {:?}", error);
        }
    });
}
