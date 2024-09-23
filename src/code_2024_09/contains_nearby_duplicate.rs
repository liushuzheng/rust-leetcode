pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let k = k as usize;
    for x in 0..n {
        for add in 1..=k {
            if x + add < n && nums[x] == nums[x + add] {
                return true;
            }
        }
    }

    false
}

#[test]
fn test_this() {
    let v = [1, 2, 3, 1].to_vec();
    let r = contains_nearby_duplicate(v, 3);
    println!("{}",r)
}