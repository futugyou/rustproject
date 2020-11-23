mod dynlib;

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
    dolly.shear();

    let foot = Inches(12);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "small"
    } else {
        "big"
    };
    println!("{:?}---{}", foot, cmp);
    let r = 1.345;
    let animal = dynlib::random_animal(r);
    println!("{:?}", animal.noise());

    for i in dynlib::fibonacce().skip(10).take(9) {
        println!("f : {}", i);
    }
}

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 10.01)
    }
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{}  says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep {
            name: name,
            naked: false,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "aaaa"
        } else {
            "bbbb"
        }
    }
    fn talk(&self) {
        println!("{}  pauses briefly {}", self.name, self.noise());
    }
}
