trait Animal {
    fn noise(&self);
}

pub struct Sheep {}

impl Animal for Sheep {
    fn noise(&self) {
        println!("mamamama!");
    }
}


fn random_animal(random_number: f64) -> Box<dyn Animal> {
    Box::new(Sheep {})
}

fn pass() {
    let mut s = String::from("hello");

    change( &mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
#[test]
fn haha() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
// 新编译器中，r3作用域在这里结束

#[test]
fn ha(){
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}",  r3);


}