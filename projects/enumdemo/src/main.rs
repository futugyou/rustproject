fn main() {
    println!("Hello, world!");
    let m = Message::Quit {};
    m.Show();
    let mm = Message::Move { x: 10, y: 23 };
    mm.Show();
    let m = Message::Write(String::from("like"));
    m.Show();
    let m = Message::ChangeColor(1, 2, 3);
    m.Show();

    let x: i8 = 6;
    let y: Option<i8> = Some(10);
    let sum = x + y.expect("");
    println!("sum is { }", sum);
    let sum = x + Option::<i8>::unwrap(y);
    println!("sum is { }", sum);
    let sum = x + y.unwrap();
    println!("sum is { }", sum);
    let sum = x + y.unwrap_or(1);
    println!("sum is { }", sum);
    if let Message::Move { x, y } = mm {
        println!("x is {}, y is {}", x, y);
    } else {
        println!("other {}", "")
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn Show(&self) {
        println!("message is {:#?}", self);
    }
}
