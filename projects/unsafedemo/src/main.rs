mod closure;
mod lib;
mod r#macro;

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    num = num + 90;
    add_to_count(num);
    unsafe {
        println!("num is {}", num);
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        println!("abs(-2) is {},static si {}", abs(-2), HELLO_WORLD);
        println!("count is {}", COUNTER);
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    assert_eq!(95, num);

    assert_eq!(
        Point { x: 1, y: 2 },
        Point { x: 0, y: 1 } + Point { x: 1, y: 1 }
    );
    assert_eq!(Millimeters(2000), Millimeters(1000) + Meters(1));

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);
    person.fly();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let answer = do_twice(returs_closure(), 5);
    println!("answer is {}", answer);

    let mac = vecdemo![1, 2, 3, 4];
    println!("mac is {:?}", mac);

    Point::hello_macro();
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello world";
static mut COUNTER: i32 = 0;
fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

use crate::closure::*;
use crate::lib::*;
use hello_world::HelloMacro;
use hello_world_derive::HelloMacro;
use std::ops::Add;

#[derive(Debug, PartialEq, HelloMacro)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(i32);
#[derive(Debug, PartialEq)]
struct Meters(i32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
