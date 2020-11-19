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
}
