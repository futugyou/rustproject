use packagedemo::eat_at_restaurant;
use packagedemo::hostingback;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    eat_at_restaurant();
    hostingback::add_to_waitlist();
    // back_of_house::Breakfast {
    //     toast: String::from("aaaa"),
    //     seasonal_fruit: String::from("aaaa"), //error  private field
    // };
}
