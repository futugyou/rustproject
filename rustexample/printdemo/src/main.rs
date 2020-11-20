fn main() {
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "a", "b", actor = "c");
    println!("now {:?} will print", Structure(2)); // now Structure(2) will print
    println!("now {} will print", Structure(2)); // now 2 wahahahah 2 will print
    println!("now {like} will print", like = Structure(2)); // now 2 wahahahah 2 will print
    println!("new {:?} will print", Deep(Structure(4)));

    let peter = Person { name: "a", age: 10 };
    println!("{:#?}", peter);

    let point = Point2d { x: 90, y: 87 };
    println!("display: {}", point);
    println!("display: {}", point.to_string());
    println!("debug: {:?}", point);
    println!("binary: {:b}", point);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    let num = 9.0987;
    println!("{}", format!("{}", num));
    println!("{}", format!("{:.2}", num));
    println!("{}", format!("{:.3}", num));
    println!("{}", format!("{:06}", 42)); //000042  only int
}

use std::fmt;
#[derive(Debug)]
struct Structure(i32);
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} wahahahah {}", self.0, self.0)
    }
}

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

//#[derive(Debug)]
struct Point2d {
    x: i32,
    y: i32,
}
impl fmt::Display for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

impl fmt::Binary for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Binary: x: {},y: {}", self.x, self.y)
    }
}

impl fmt::Debug for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}
