//  https://leetcode.cn/problems/minimum-cuts-to-divide-a-circle/
pub fn number_of_cuts(n: i32) -> i32 {
    match n {
        1 => 0,
        _ => match n & 1 {
            0 => n / 2,
            _ => n,
        },
    }
}

#[test]
fn test_fn() {
    let i = number_of_cuts(1);
    println!("the step is {}", i);
}