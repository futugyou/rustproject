pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("this is pilot");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("this is wizard");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("this is human");
    }
}

use std::fmt;
pub struct Wrapper(pub Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{}]", self.0.join(", "))
    }
}
