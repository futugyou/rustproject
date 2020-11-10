use std::collections::HashMap;
use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");
    generate_work(2, 19);
}

fn generate_work(intensity: u32, rand_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("wait.....");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("{}", expensive_closure.value(intensity));
    println!("{}", expensive_closure.value(rand_number));
}
struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + std::cmp::Eq + std::hash::Hash,
{
    calc: T,
    value: Option<U>,
    map: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + std::cmp::Eq + std::hash::Hash,
{
    pub fn new(calc: T) -> Cacher<T, U> {
        Cacher {
            calc,
            value: None,
            map: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: U) -> U {
        match self.map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                self.map.insert(arg, v);
                v
            }
        }
    }
}
