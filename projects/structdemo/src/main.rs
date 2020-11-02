fn main() {
    println!("Hello, world!");
    let r = Rectangle {
        width: 20,
        height: 10,
    };
    println!("result is {:#?}", r);
    println!("area is {}", r.area());
    println!("cale is {}", calc(r));
    Rectangle::show();
}

fn calc(r: Rectangle) -> u32 {
    r.height * r.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show() {
        println!("{}", "this is show()");
    }
}
