#[test]
fn iterator_demon() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(Some(&1), v1_iter.next());
    assert_eq!(Some(&2), v1_iter.next());
    assert_eq!(Some(&3), v1_iter.next());
    assert_eq!(None, v1_iter.next());
}
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter().sum::<u32>();
    assert_eq!(v1_iter, 6);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v1_iter: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v1_iter, vec![2, 3, 4]);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 3 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next() {
    let mut counter = Counter::new();
    assert_eq!(Some(1), counter.next());
    assert_eq!(Some(2), counter.next());
    assert_eq!(None, counter.next());
}

#[test]
fn counter_test_2() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(0))
        .map(|(a, b)| a * b)
        .filter(|x| *x > 0)
        .sum();
    assert_eq!(sum, 5);
}
