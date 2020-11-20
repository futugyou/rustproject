use core::convert::TryFrom;
use std::convert::TryInto;

fn main() {
    let num = Number::from(20);
    println!("Hello, world! {:?}", num);
    let int = 5;
    let num: Number = int.into();
    println!("Hello, world! {:?}", num);

    assert_eq!(EvenNumber::try_from(10), Ok(EvenNumber(10)));
    assert_eq!(EvenNumber::try_from(11), Err(()));

    let result: Result<EvenNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));
    matchmothed();

    let list = vec![1, 2, 3];
    list.iter().find(|&&x| x == 2);
    list.iter().any(|&x| x == 2);
    let closure = move |a| list.contains(a);
    println!("{}", closure(&1));
    println!("{}", closure(&2));
    //println!("{}", list.len()); //borrow of moved value: `list`

    fndemo();

    orderdemo();
}

fn orderdemo() {
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("acc is {}", acc);

    let sum_of_odd_number: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |accc, n_squared| accc + n_squared);
    println!("result is {}", sum_of_odd_number);
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn fndemo() {
    use std::mem;
    let greeting = "hello";
    let mut farewell = String::from(""); //"goodbye".to_owned();
    let diary = || {
        println!("i sad {}", greeting);
        farewell.push_str("----");
        println!("then {}", farewell);
        println!("end ");
        mem::drop(farewell);
    };
    apply(diary);
    //apply(diary); //value used here after move
    let d = |x| 2 * x;
    let x = 7;
    let print = move || println!("{}", x);
    apply(print);
    apply(print);
    println!("{}", apply_to_3(d));
    println!("{}", apply_to_3(d));
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn matchmothed() {
    let reference = &4;
    match reference {
        &val => println!("get destructing: {:?}", val),
    }
    match *reference {
        val => println!("{:?}", val),
    }
    match *reference {
        ref f => println!("{:?}", f),
    }
    let mut value = 6_i32;
    match value {
        ref mut f => {
            *f *= 2;
            println!("{:?}", f)
        }
    }
    match value {
        0 => println!("not "),
        n @ 1..=11 => println!("small {}", n),
        n @ 12..=100 => println!("small too {}", n),
        n => println!("other {}", n),
    }
    println!("{:?}", value);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);
impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as std::convert::TryFrom<i32>>::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}
