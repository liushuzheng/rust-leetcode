fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}




// fn something<T>(val: T)
//     where
//         Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
// {
//     //
// }

fn main() {
    // something([0u8; 0]); // ok
    // something([0u8; 512]); // ok
    // something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
}

// ---

pub enum Assert<const CHECK: bool> {
    //
}

pub trait IsTrue {
    //
}

impl IsTrue for Assert<true> {
    //
}