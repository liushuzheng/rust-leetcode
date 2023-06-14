// https://leetcode.cn/problems/fibonacci-number/?envType=study-plan-v2&envId=dynamic-programming

pub fn fib(n: i32) -> i32 {
    let mut v = Vec::with_capacity(n as usize);
    v.push(0);
    v.push(1);
    for i in 2..=n {
        let pre = v.get((i - 1) as usize).unwrap();
        let pre_two = v.get((i - 2) as usize).unwrap();
        v.push(pre + pre_two);
    }
    let &value = v.get(n as usize).unwrap();
    value
}

#[test]
fn test_fb() {
    let i = fib(4);
    println!("value is {}",i);
}