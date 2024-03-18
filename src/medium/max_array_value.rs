pub fn max_array_value(nums: Vec<i32>) -> i64 {
    let mut sum = nums[nums.len() - 1] as i64;
    for i in (0..nums.len() - 1).rev() {
        sum = if nums[i] as i64 <= sum {
            sum + nums[i] as i64
        } else {
            nums[i] as i64
        }
    }
    sum
}

#[test]
fn test_fn() {
    let v = vec![2, 3, 7, 9, 3];
    let sum = max_array_value(v);
    println!("{}", sum)
}