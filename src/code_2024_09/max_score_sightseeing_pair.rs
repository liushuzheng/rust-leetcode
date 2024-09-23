pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut left_max = values[0];
    for i in 1..values.len() {
        result = result.max(left_max + values[i] - i as i32);
        left_max = left_max.max(values[i] + i as i32);
    }
    result
}


#[test]
fn test_fn() {
    let v = vec![8, 1, 5, 2, 6];
    let i = max_score_sightseeing_pair(v);
    println!("{}", i)
}