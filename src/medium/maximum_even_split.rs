/// https://leetcode.cn/problems/maximum-split-of-positive-even-integers/
pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
    let mut v = vec![];
    if final_sum & 1 == 1 {
        return v;
    }
    let mut sum = 2;
    let mut step = 2;

    while sum <= final_sum {
        v.push(step);
        step += 2;
        sum += step;
    }
    let i = final_sum - sum + step;
    let last = v.pop().unwrap();
    v.push(last + i);
    return v;
}

#[test]
fn test_this() {
    let vec = maximum_even_split(32);
    println!("{:#?}", vec);
}