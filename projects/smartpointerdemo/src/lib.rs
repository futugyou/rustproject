pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messager> {
    messager: &'a T,
    value: u32,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &T) -> LimitTracker<T> {
        LimitTracker { messager, value: 0 }
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;
        match value {
            1 => self.messager.send("this is 1"),
            2 => self.messager.send("this is 2"),
            _ => self.messager.send("this is other"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessager {
        send_messages: RefCell<Vec<String>>,
    }
    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                send_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messager for MockMessager {
        fn send(&self, message: &str) {
            self.send_messages.borrow_mut().push(String::from(message));
        }
    }
    #[test]
    fn test_send_message() {
        let messager = MockMessager::new();
        let mut tracker = LimitTracker::new(&messager);
        tracker.set_value((2));
        tracker.set_value((2));
        tracker.set_value((2));
        assert_eq!(2, messager.send_messages.borrow().len());
    }
}
