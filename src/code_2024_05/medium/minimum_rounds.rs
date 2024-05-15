use std::collections::HashMap;

pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    
    let mut times = 0;
    let map = tasks.iter()
        .fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
    for &x in map.values() {
        if x < 2 {
            return -1;
        }
        let t = x / 3;
        if x % 3 == 0 {
            times += t;
        } else {
            times += (t + 1);
        }
    }
    times
}

#[test]
fn test_func() {
    let v = vec![2,3,3];
    let i = minimum_rounds(v);
    println!("{}",i)
}