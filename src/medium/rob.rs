use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let n = nums.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = nums[0];
    for i in 2..=n {
        dp[i] = max(dp[i - 1], dp[i - 2] + nums[i - 1]);
    }
    return dp[n];
}

#[test]
fn test_fn() {
    let nums = vec![1, 2, 3, 1];
    let i = rob(nums);
    println!("{i}")
}