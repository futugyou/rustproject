use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    rerefcell();
}

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// fn rerefcell() {
//     let value = Rc::new(RefCell::new(5));
//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(57)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(13)), Rc::clone(&a));
//     *value.borrow_mut() += 10;
//     println!("{:?}", a);
//     println!("{:?}", b);
//     println!("{:?}", c);
// }
#[derive(Debug)]
enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil,
}

fn rerefcell() {
    let value = RefCell::new(5);
    let a = Rc::new(Cons(RefCell::clone(&value), Rc::new(Nil)));
    let b = Cons(RefCell::new(57), Rc::clone(&a));
    let c = Cons(RefCell::new(13), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
