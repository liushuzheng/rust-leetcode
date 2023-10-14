/// https://leetcode.cn/problems/single-number/

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];
    if nums.len() > 1 {
        for i in 1..nums.len() {
            result = nums[i] ^ result;
        }
    }
    result
}

#[test]
fn test_single_number() {
    let v = vec![4,1,2,1,2];
    let i = single_number(v);
    assert_eq!(4, i);
}