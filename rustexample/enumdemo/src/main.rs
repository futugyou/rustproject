use crate::List::*;

enum List {
    Cons(i32, Box<List>),
    Nil,
}
impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: i32) -> List {
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => format!("{} {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("lenght {}", list.len());
    println!("{}", list.stringify());

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;
    println!("size of x in bytes:{}", std::mem::size_of_val(&x));
    println!("size of y in bytes:{}", std::mem::size_of_val(&y));
    println!("size of z in bytes:{}", std::mem::size_of_val(&z));
    println!("size of i in bytes:{}", std::mem::size_of_val(&i));
    println!("size of f in bytes:{}", std::mem::size_of_val(&f));
}
