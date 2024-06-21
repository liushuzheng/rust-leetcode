use tokio;

// fn main() {
//     // println!("Hello, world!");
//     // 创建runtime
//     let rt = tokio::runtime::Runtime::new().unwrap();
// }

use std::convert::TryInto;

fn main() {
    let a: u8 = 10;
    let b: u16 = 1500;

    let b_: u8 = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }

    let s1: Box<str> = "Hello there!".into();
}
