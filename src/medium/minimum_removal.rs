/// https://leetcode.cn/problems/removing-minimum-number-of-magic-beans/description/
pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
    beans.sort();
    let len = beans.len();
    let total: i64 = beans.iter().map(|&i| i as i64).sum();
    let mut res = total;
    for (i, &item) in beans.iter().enumerate() {
        res = res.min(total - item as i64 * (len - i) as i64)
    }
    res
}


#[test]
fn test_function() {
    let mut v = vec![4, 1, 6, 5];
    let i = minimum_removal(v);
    println!("{}", i)
}