use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    demo1();
    println!("-----------");
    demo2();
    println!("-----------");
    demo3();
}
fn demo3() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result is {}", counter.lock().unwrap());
}

fn demo2() {
    let (tx, rx) = mpsc::channel();
    for i in 1..10 {
        let tx = mpsc::Sender::clone(&tx);
        std::thread::spawn(move || {
            tx.send(format!("thread send {}", i)).unwrap();
        });
    }
    // this sned must in spawn ,if not there will be a deadlock
    std::thread::spawn(move || {
        tx.send(format!("thread send {}", " ")).unwrap();
    });
    for r in rx {
        println!("recv :{}", r);
    }
}
fn demo1() {
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("thred number :{}  ", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("main number :{} ", i);
        thread::sleep(Duration::from_millis(1));
    }
    match handle.join() {
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{:?}", e),
    };
}
