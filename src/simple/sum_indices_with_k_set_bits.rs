pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter().enumerate().fold(0, |ans, (i, v)| {
        ans + (i.count_ones() as i32 == k) as i32 * v
    })
}


#[test]
fn test_1() {
    let i: usize = 5;
    println!("{}", i.count_ones())
}