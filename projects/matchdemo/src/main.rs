fn main() {
    let color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(c) = color {
        println!("like color is {}", c);
    } else if is_tuesday {
        println!("tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("30");
        } else {
            println!("less than 30")
        }
    } else {
        println!("other");
    }

    let mut stack = Vec::new();
    stack.push(IndexItem {});
    stack.push(IndexItem {});
    stack.push(IndexItem {});
    // while let Some(top) = stack.pop() {
    //     println!("num is {:?}", top);
    // }
    for (index, item) in stack.iter().enumerate() {
        println!("index is {} ,num is {:?}", index, item);
    }

    //let x: Option<u32> = None;
    let x = Some(49);
    let y = 10;
    match x {
        Some(50) | Some(49) => println!("x is 50"),
        Some(n) if n == y => println!("y is {}", y),
        _ => println!("other x"),
    }
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
#[derive(Debug)]
struct IndexItem {}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}
