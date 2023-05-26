//给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，
// 计算按此排列的柱子，下雨之后能接多少雨水

//输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
// 输出：6
// 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，
// 可以接 6 个单位的雨水（蓝色部分表示雨水）。

use std::ops::Index;

pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut res = 0;
    let (mut l, mut r, mut left, mut right) = (0, n - 1, 0, 0);
    while l < r {
        let (a, b) = (height[l], height[r]);
        left = left.max(a);
        right = right.max(b);
        if left < right {
            res += left - a;
            l += 1
        } else {
            res += right - b;
            r -= 1;
        }
    }
    res
}

#[test]
fn test_trap() {
    let v = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    let i = trap(v);
    println!("trap value is {}", i);
}

