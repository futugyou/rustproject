fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    // let _word = first_word(&s);
    // s.clear();

    let _word = first_word1(&s); // -- immutable borrow occurs here
    s.clear(); //mutable borrow occurs here
    println!("Hello, world! {}", _word); // ----- immutable borrow later used here

    // 这段代码很有意思，如果没有println这行，那么不会报错，有了就会报错
    // 但是原本println之前的代码就已经是借用了immutable和mutable，为什么只有在println这行用到了 word的时候才报错。
    // 造成个结果的原因是: 引用的作用域是从引用位置开始到最后一次引用之间
    // 如果没有println那行，word在赋值那行结束就没用了，所以可以s.clear()
    // 看下面两个例子ok_one(),error_one()
}

fn ok_one() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn error_one() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    println!("{} and {}", r1, r2);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
