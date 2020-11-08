#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
    #[ignore]
    #[test]
    fn it_works2() {
        assert_ne!(1 + 2, 4);
    }

    #[test]
    fn larger_can_hold() {
        let larger = Rectangle {
            length: 9,
            width: 8,
        };
        let small = Rectangle {
            length: 4,
            width: 1,
        };
        assert!(larger.can_hold(&small));
        assert!(!small.can_hold(&larger));
    }
    #[test]
    #[should_panic]
    fn big_than_100() {
        Guess::new(1000);
    }
    #[test]
    fn reslut_work() -> Result<(), String> {
        if true {
            Ok(())
        } else {
            Err(String::from("not ok"))
        }
    }
    // #[test]
    // fn other() {
    //     panic!("error ");
    // }
}
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub struct Guess {
    #[warn(dead_code)]
    value: u32,
}
impl Guess {
    pub fn new(v: u32) -> Guess {
        if v < 1 || v > 100 {
            panic!("too big or too small");
        } else {
            Guess { value: v }
        }
    }
}
