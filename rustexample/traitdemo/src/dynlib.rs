use std::ops;

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

pub struct Fibonacci {
    curr: u32,
    next: u32,
}
impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

pub fn fibonacce() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

pub struct Big {}
pub struct Cow {}

impl ops::Add<Big> for Cow {
    type Output = Cow;
    fn add(self, b: Big) -> Self::Output {
        println!("add {}{}", self.noise(), b.noise());
        self
    }
}

impl Drop for Big {
    fn drop(&mut self) {
        println!("drop {}", self.noise());
    }
}

pub trait AllAnimal {
    fn noise(&self) -> &'static str;
}

impl AllAnimal for Big {
    fn noise(&self) -> &'static str {
        "ppppp"
    }
}

impl AllAnimal for Cow {
    fn noise(&self) -> &'static str {
        "oooooo"
    }
}

pub fn random_animal(random_number: f64) -> Box<dyn AllAnimal> {
    if random_number < 0.5 {
        Box::new(Big {})
    } else {
        Box::new(Cow {})
    }
}
