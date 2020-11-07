fn main() {
    let number_list = vec![1, 2, 5, 5, 6, 1234, 234, 234];
    let big = largest1(&number_list);
    println!("big is {}", big);
    let act = Actice {
        headline: String::from("a"),
        location: String::from("b"),
    };
    println!("actice is {}", act.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest1<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("")
    }
}
pub struct Actice {
    pub headline: String,
    pub location: String,
}
impl Summary for Actice {
    fn summarize(&self) -> String {
        format!("Actice : {}-{}", self.headline, self.location)
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
