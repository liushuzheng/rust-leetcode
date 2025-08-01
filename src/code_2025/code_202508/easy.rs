pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    result.push(vec![1]);

    for i in 1..num_rows as usize {
        let mut cow = vec![];
        for j in 0..=i {
            if j == 0 || j == i {
                cow.push(1);
            }else {
                cow.push( result[i-1][j-1] + result[i-1][j]);
            }
        }
        result.push(cow);
    }
    result
}
