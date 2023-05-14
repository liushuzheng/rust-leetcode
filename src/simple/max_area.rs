//给定一个长度为 n 的整数数组height。有n条垂线，第 i 条线的两个端点是(i, 0)和(i, height[i])。
//
// 找出其中的两条线，使得它们与x轴共同构成的容器可以容纳最多的水。
//
// 返回容器可以储存的最大水量。
//
// 说明：你不能倾斜容器。

//输入：[1,8,6,2,5,4,8,3,7]
// 输出：49 
// 解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。
// 在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为49。
use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let (mut l, mut r) = (0, height.len() - 1);
    while l < r {
        if let (Some(v1), Some(v2)) = (height.get(l), height.get(r)) {
            max_area = cmp::max(max_area, cmp::min(v1, v2) * (r - l) as i32);
            if v1 < v2 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    max_area
}

#[test]
fn testd() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let area = max_area(height);
    println!("area value is {}", area);
}
