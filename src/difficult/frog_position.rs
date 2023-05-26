pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    1_f64 / 6_f64
}

#[test]
fn test_frog() {
    // let position = frog_position(1, vec![], 4, 3);
    // println!("value is {}",position)
    let mut count: i128 = 1;
    for i in 1..30 {
        count *= i;
    }
    println!("count is ${count}")
}