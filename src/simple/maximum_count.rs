/// let mut pos = 0;
/// let mut neg = 0;
/// for x in nums {
///     if x > 0 {
///         pos += 1;
///     }else if x <0 {
///         neg +=1;
///     }
/// }
/// max(pos, neg)
pub fn maximum_count(nums: Vec<i32>) -> i32 {
    // nums.partition_point(|&x| x < 0)
    //     .max(nums.len() - nums.partition_point(|&x| x < 1)) as i32
    let iter = nums.iter();
    // let iter1 = nums.into_iter();

    0

}