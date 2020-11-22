//rustc src/main.rs --extern rary=liblib.rlib --edition=2018 && ./executable
fn main() {
    public_function();
    indirect_access();

    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    let x = X;
    let y = Y;
    x.double_drop(y);

    let n1 = 3;
    let n2 = 10;
    let container = Container(n1, n2);
    println!("{}:{}:{}", n1, n2, container.contains(&n1, &n2));
    println!("{}", container.first());
    println!("{}", container.last());
    println!("{}", difference(&container));

    let _tup1: PhantomTuple<char, f32> = PhantomTuple('a', PhantomData);
    let _tup2: PhantomTuple<char, f64> = PhantomTuple('a', PhantomData);
    //println!("{}", _tup1 == _tup2);

    let point = Point { x: 0, y: 0 };
    let _copy1 = {
        let Point { x: ref ref_x, y: _ } = point;
        *ref_x
    };
    let _copy2 = {
        let Point { x: ref_x, y: _ } = point;
        ref_x
    };
    println!("{}{}", _copy1, _copy2);

    let mut point2 = point;
    {
        let Point {
            x: _,
            y: ref mut mut_y,
        } = point2;
        *mut_y = 2;
    }
    println!("{:?}{:?}", point, point2);
}

mod lib;
use crate::lib::*;

struct Empty;
struct Null;

struct X;
struct Y;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {
        println!("ppp");
    }
}
struct Container(i32, i32);

trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    fn contains(&self, n1: &<Self as Contains>::A, n2: &<Self as Contains>::B) -> bool {
        (&self.0 == n1) && (&self.1 == n2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);
#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
