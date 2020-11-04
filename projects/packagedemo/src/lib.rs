#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 5);
    }
}

pub mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("aaaa"),
            }
        }
    }
}
mod front_of_house;
pub use crate::front_of_house::hosting as hostingback;

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    hostingback::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("hello");
    meal.toast = String::from("wolrd");
    println!("{:#?}", meal);
}
