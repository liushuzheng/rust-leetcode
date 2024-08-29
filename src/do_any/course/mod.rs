fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}


pub trait Animal {
    type Item: std::fmt::Display;
    fn print_item(item: &Self::Item) {
        println!("{}", item);
    }
}


impl Animal for i32 {
    type Item = i32;
    
}

#[test]
fn main() {
    let a = 32;
    <i32 as Animal>::print_item(&a);
    
     let s =  r"hello
     fataf
     aheg";
    println!("{}",s)
}