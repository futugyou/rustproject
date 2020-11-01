fn main() {
    println!("Hello, world!");
    let x = {
        let a = 1;
        add(a)
    };
    println!("x is {}", x);

    ifcondition();
    loop_println();
    while_println();
    for_println();
}

fn for_println() {
    let a = [1, 2, 3, 4, 5, 6];
    for elem in a.iter() {
        println!("for elem is {}", elem)
    }
    let b = 1..5;
    println!("range a lenght is {}", b.len()); //4
    for elem in b.rev() {
        println!("for elem rev() is {}", elem)
    }
}

fn while_println() {
    let mut while_num = 3;
    while while_num != 0 {
        while_num -= 1;
    }
    println!("while_num is {}", while_num)
}

fn loop_println() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop is {}", result);
}

fn add(x: u32) -> u32 {
    x + 1
}

fn ifcondition() {
    let condition = true;
    let num = if condition { add(11) } else { 6 };
    println!("num is {}", num)
}
