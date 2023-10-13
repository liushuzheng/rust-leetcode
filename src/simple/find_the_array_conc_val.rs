/// https://leetcode.cn/problems/find-the-array-concatenation-value/


pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut sum: i64 = 0;
    while start < end {
        let num_len = nums[end].to_string().len() as u32;
        sum += nums[end] as i64 + nums[start] as i64 * i64::pow(10, num_len);
        start += 1;
        end -= 1;
    }
    if nums.len() % 2 == 1 {
        sum += nums[nums.len() / 2] as i64;
    }

    return sum;
}

#[test]
fn test_func() {
    let v = vec![5,14,13,8,12];
    let i = find_the_array_conc_val(v);
    println!("测试返回的串联数字是{}", i);
}
