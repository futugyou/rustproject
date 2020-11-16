#[macro_export]
macro_rules! vecdemo {
    ($($x:expr),*) => {
        {
        let mut temp=Vec::new();
        $(
            temp.push($x);
        )*
        temp
    };}
}
