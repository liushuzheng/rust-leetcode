pub fn pivot_index(nums: Vec<i32>) -> i32 {
    if nums.is_empty() || nums.len() == 1 {
        return 0;
    }
    let sum = nums.iter().sum::<i32>();
    let mut left_sum = 0;
    for (index, &value) in nums.iter().enumerate() {
        left_sum += value;
        if (sum - value) % 2 != 0 {
            continue;
        }

        if left_sum == (sum - left_sum + value) {
            return index as i32;
        }
    }
    -1
}

#[test]
fn test_this() {
    let v = vec![1, 7, 3, 6, 5, 6];
    let i = pivot_index(v);
    println!("{}", i)
}