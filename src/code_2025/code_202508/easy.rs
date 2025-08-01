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

pub fn generate2(num_rows: i32) -> Vec<Vec<i32>> {
    let n = num_rows as usize;
    let mut c = vec![vec![]; n];
    for i in 0..n {
        c[i].resize(i + 1, 1);
        for j in 1..i {
            // 左上方的数 + 正上方的数
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    c
}


