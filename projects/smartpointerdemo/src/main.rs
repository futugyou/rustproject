fn main() {
    let b = Box::new(5);
    println!("b={}", b);
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:#?}", list);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*(y.deref()), 5);
    assert_eq!(*(y), 5);

    let m = MyBox::new(String::from("li lei"));
    hello(&m);
    hello(&(*m)[..]);

    let _c1 = CustomPointer {
        data: String::from("one"),
    };
    let c2 = CustomPointer {
        data: String::from("two"),
    };
    drop(c2);
    println!("main end");

    let a1 = Rc::new(List2::Cons(
        1,
        Rc::new((List2::Cons(2, Rc::new(List2::Nil)))),
    ));
    println!("rc count :{}", Rc::strong_count(&a1));
    let _b1 = List2::Cons(4, Rc::clone(&a1));
    println!("rc count :{}", Rc::strong_count(&a1));
    {
        let _c1 = List2::Cons(8, Rc::clone(&a1));
        println!("rc count :{}", Rc::strong_count(&a1));
    }
    println!("rc count :{}", Rc::strong_count(&a1));
}

use std::rc::Rc;
#[derive(Debug)]
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomPointer {
    data: String,
}
impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("dropping data {}", self.data);
    }
}
