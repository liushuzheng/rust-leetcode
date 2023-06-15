//  https://leetcode.cn/problems/n-th-tribonacci-number/?envType=study-plan-v2&envId=dynamic-programming

pub fn tribonacci(n: i32) -> i32 {
    let mut v = Vec::with_capacity(n as usize);
    v.push(0);
    v.push(1);
    v.push(1);
    for i in 3..=n {
        let pre = v.get((i - 1) as usize).unwrap();
        let pre_two = v.get((i - 2) as usize).unwrap();
        let pre_three = v.get((i - 3) as usize).unwrap();
        v.push(pre + pre_two + pre_three);
    }
    let &value = v.get(n as usize).unwrap();
    value
}

#[test]
fn test_fn() {
    let i = tribonacci(25);
    println!("value is {}", i);
}