use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};

fn main() {
    threaddemo();
    println!("");
    channeldemo();
    println!("");
    pathdemo();
    println!("");
    filedemo();
    println!("");
}

fn filedemo() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("error {} : {}", display, e),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("error {} :{}", display, e),
        Ok(_) => print!("{} contains:\n{}\n\n", display, s),
    }
    let path = Path::new("hello2.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(e) => panic!("error {} : {}", display, e),
        Ok(file) => file,
    };
    match file.write_all(s.as_bytes()) {
        Err(e) => panic!("error {} :{}", display, e),
        Ok(_) => print!("{} contains:\n{}\n\n", display, s),
    }

    if let Ok(lines) = read_lines("hello.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pathdemo() {
    let path = Path::new(".");
    let _display = path.display();
    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not valid"),
        Some(s) => println!("new path os {}", s),
    }
}

fn channeldemo() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..3 {
        let thread_tx = tx.clone();
        let child = std::thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} is finished", id);
        });
        children.push(child);
    }

    let mut ids = Vec::with_capacity(3 as usize);
    for _ in 0..3 {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("error");
    }
    println!("{:?}", ids);
}

fn threaddemo() {
    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    let mut children = vec![];
    let chunked_data = data.split_whitespace();
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data_segment {} is {}", i, data_segment);

        children.push(std::thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("number error"))
                .sum();
            println!("processed segment {} ,result = {}", i, result);
            result
        }));
    }

    let mut inermediate_sums = vec![];
    for child in children {
        let intermediate_sum = child.join().unwrap();
        inermediate_sums.push(intermediate_sum);
    }
    let final_result = inermediate_sums.iter().sum::<u32>();
    println!("last sum is {}", final_result);
}
