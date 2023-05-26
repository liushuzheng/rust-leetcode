//假设你正在爬楼梯。需要 n阶你才能到达楼顶。
//
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

//
// 示例 1：
//
// 输入：n = 2
// 输出：2
// 解释：有两种方法可以爬到楼顶。
// 1. 1 阶 + 1 阶
// 2. 2 阶
// 示例 2：
//
// 输入：n = 3
// 输出：3
// 解释：有三种方法可以爬到楼顶。
// 1. 1 阶 + 1 阶 + 1 阶
// 2. 1 阶 + 2 阶
// 3. 2 阶 + 1 阶

pub fn climb_stairs(n: i32) -> i32 {
    let mut v: Vec<i32> = Vec::with_capacity(n as usize);
    v.insert(0, 1);
    v.insert(1, 1);
    for i in 2..(n + 1) {
        let x1 = v.get((i - 1) as usize).unwrap();
        let x2 = v.get((i - 2) as usize).unwrap();
        v.insert(i as usize, x1 + x2)
    }

    if let Some(value) = v.last() {
        return *value;
    }
    0
}


#[test]
fn test_climb_stairs() {
    let n = 4;
    let step = climb_stairs(n);
    println!("{} 阶楼梯需要{}步", n, step);
}