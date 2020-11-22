fn main() {
    println!("Hello, world!");
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
}

fn failed_borrow<'a>() {
    let _x = 13;
    // let y: &'a i32 = &_x;
    // `_x` does not live long enough
    // borrowed value does not live long enoughrustc(E0597)
    // main.rs(5, 18): lifetime `'a` defined here
    // main.rs(7, 12): type annotation requires that `_x` is borrowed for `'a`
    // main.rs(7, 22): borrowed value does not live long enough
    // main.rs(8, 1): `_x` dropped here while still borrowed
}

fn print_one<'a>(x: &'a i32) {
    println!("{}", x);
}

use std::fmt::Debug;
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T)
where
    T: Debug,
{
    println!("t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("ref t is {:?}", t);
}
