fn main() {
    vecdemo();
    println!("Hello, world!");
    stringdemo();
    mapdemo();
    arcdemo();
}
fn arcdemo() {
    use std::sync::Arc;
    use std::thread;

    let apple = Arc::new("an apple");
    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}

fn mapdemo() {
    use std::collections::HashMap;
    let mut contacts = HashMap::new();
    contacts.insert("a", "123");
    contacts.insert("b", "234");
    contacts.insert("c", "345");
    contacts.insert("d", "456");

    match contacts.get("a") {
        Some(number) => println!("hashmap get :{}", number),
        _ => println!("{}", "nothing"),
    }
    contacts.remove("a");
    for (c, n) in contacts.iter() {
        println!("{} is {}", c, n);
    }
    use std::collections::HashSet;
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![3i32, 4, 5].into_iter().collect();
    a.insert(4);
    a.insert(4);
    println!("hashset a : {:?}", a);
    println!("hashset b : {:?}", b);
    println!("union : {:?}", a.union(&b).into_iter());
    println!("difference : {:?}", a.difference(&b).into_iter());
    println!("intersection : {:?}", a.intersection(&b).into_iter());
    println!(
        "symmetric_difference : {:?}",
        a.symmetric_difference(&b).into_iter()
    );
}

fn stringdemo() {
    let pangram: &str = "the quick brown fox jumps over the lazy dog";
    for word in pangram.split_whitespace().rev() {
        println!(">{}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("{}", string);
    let chars_to_trim: &[char] = &[' ', ','];
    let trim_str: &str = string.trim_matches(chars_to_trim);
    println!("{}", trim_str);
    println!("{}", trim_str.replace("a", "b"));
}

fn vecdemo() {
    let iter: Vec<i32> = (0..10).collect();
    println!("{:?}", iter);

    let mut xs = vec![1i32, 2, 3];
    xs.push(0);
    xs.pop();

    for i in xs.iter() {
        println!(">{}", i);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("{}:{}", i, x);
    }
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("{:?}", xs);
}
