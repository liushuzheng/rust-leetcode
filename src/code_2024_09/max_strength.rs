// [3,-1,-5,2,5,-9]
pub fn max_strength(nums: Vec<i32>) -> i64 {
    let mut n = nums.clone();
    n.sort();
    n.retain(|&x| x != 0);
    if let Some(index) = n.iter().rposition(|&x| x < 0) {
        if index % 2 == 0 {
            n.remove(index);
        }
    }
    if (n.is_empty()) {
        return 0;
    }

    n.iter().fold(1, |acc, &x| acc * x as i64)
}