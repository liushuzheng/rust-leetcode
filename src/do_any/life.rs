#[derive(Debug)]
struct Foo;

impl Foo {
    // fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
    //     &'a *self
    // }
    // fn share<'a>(&'a self) {}
}

#[test]
fn test_foo() {
    let a = (1, 2, 3, "str");
    println!("{},{},{},{}", a.0, a.1, a.2, a.3)
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

