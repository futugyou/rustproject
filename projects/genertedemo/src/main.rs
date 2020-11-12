fn main() {
    println!("Hello, world!");
    // vecdemo();
    // stringdemo();
    mapdemo();
}

fn mapdemo() {
    use std::collections::HashMap;
    let mut s = HashMap::new();
    s.insert(String::from("one"), 10);
    s.insert(String::from("two"), 11);
    for (_, value) in &mut s {
        *value = *value + 1;
    }
    println!("{:#?}", s);

    println!("{} {}", "", "");
    let teams = vec![String::from("one"), String::from("two")];
    let scores = vec![String::from("11"), String::from("22")];
    let mut s: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    for (key, value) in &mut s {}
    println!("{:#?}", s);
}

fn stringdemo() {
    let mut s1 = String::from("one");
    let s2 = "two";
    s1.push_str(s2);
    s1.push('s');
    println!("s1 is {}, s2 is {}", s1, s2);
    for c in s1.chars() {
        println!("{}", c);
    }
    for c in s1.bytes() {
        println!("{}", c);
    }
}

fn vecdemo() {
    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:#?}", v);
    let mut v = vec![1, 2, 40];
    let third = &v[2];
    user2(third);
    //v.push(10);  //error
    println!("{:#?}", third);
    let third = v[2];
    v.push(10);
    println!("{:#?}", third);
    user(third);
    println!("{:#?}", v.get(100)); //None
                                   //println!("{:#?}", v[100]); //index out of bounds: the len is 3 but the index is 100
    for i in &mut v {
        *i = *i + 2;
    }
    println!("{:#?}", v);
    let c = vec![1, 3, 4, 5];
    c.len();
    c.len();
}

fn user(_v: u32) {}

fn user2(_v: &u32) {}
