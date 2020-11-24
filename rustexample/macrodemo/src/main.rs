macro_rules! say_hello {
    () => {
        println!("hello");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("you called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! test {
    ($left:expr;and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr;or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! find_mix {
    ($x:expr)=>($x);
    ($x:expr,$($y:expr),+) => {
        std::cmp::min($x,find_mix!($($y),+))
    };
}

macro_rules! calculate {
    (eval $e:expr) => {
        {{
            let val:usize=$e;
            println!("{} = {}",stringify!{$e},val);
        }}
    };
    (eval $e:expr,$(eval $es:expr),+)=>{{
        calculate!{eval $e}
        calculate!{$(eval $es),+}
    }};
}

fn main() {
    foo();
    bar();
    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
    say_hello!();

    test!(1i32+1==2i32;and 2i32*2==4i32);
    test!(true; or false);

    println!("{}", find_mix!(1u32));
    println!("{}", find_mix!(1u32 + 2, 2u32));
    println!("{}", find_mix!(1u32, 2u32 * 3, 4u32));

    calculate! {
        eval 1+2
    }
    calculate! {
        eval (1+2)*(3/4),
        eval 2+4
    }
}
