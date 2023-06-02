//给你一个由正整数组成的整数数组 nums ，返回其中可被 3 整除的所有偶数的平均值。
//
// 注意：n 个元素的平均值等于 n 个元素 求和 再除以 n ，结果 向下取整 到最接近的整数。
//
//
// 示例 1：
//
// 输入：nums = [1,3,6,10,12,15]
// 输出：9
// 解释：6 和 12 是可以被 3 整除的偶数。(6 + 12) / 2 = 9 。
// 示例 2：
//
// 输入：nums = [1,2,4,7,10]
// 输出：0
// 解释：不存在满足题目要求的整数，所以返回 0 。

pub fn average_value(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut count = 0;
    for x in nums {
        if x % 3 == 0 && x & 1 == 0 {
            sum += x;
            count += 1;
        }
    }
    if count > 0 {
        return sum / count;
    }
    0
}

pub fn average(nums: Vec<i32>) -> i32 {
    let (sum, cnt) = nums.iter()
        .filter(|&&x| x % 6 == 0)
        .fold((0, 0), |(sum, cnt), &x| (sum + x, cnt + 1));
    if cnt == 0 {
        0
    } else {
        sum / cnt
    }
}

#[test]
fn test_fn() {
    let v = vec![1, 2, 4, 7, 10];
    let i = average(v);
    println!("value is {}", i)
}