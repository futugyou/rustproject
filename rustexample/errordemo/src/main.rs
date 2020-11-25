#![allow(dead_code)]
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}
#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("i love {:?}", food),
        None => println!("no"),
    }
}
fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    print(multiply("10", "2"));

    let strings = vec!["aaaa", "123", "234"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!(" Result<Vec<_>, _>: {:?}", numbers);

    let strings = vec!["aaaa", "123", "234"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!(" Vec<_> : {:?}", numbers);

    let strings = vec!["aaaa", "123", "234"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("filter_map: {:?}", numbers);

    let strings = vec!["aaaa", "123", "234"];
    let (numbers, error): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("numbers: {:?}", numbers);
    println!("error: {:?}", error);

    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let error: Vec<_> = error.into_iter().map(Result::unwrap_err).collect();
    println!("numbers: {:?}", numbers);
    println!("error: {:?}", error);
}
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?; //try!() use of deprecated `try` macro
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}
fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error:{}", e),
    }
}
