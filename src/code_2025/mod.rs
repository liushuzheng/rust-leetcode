pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
    let (l, r, t, b) = (0, n, 0, n);
    let (mut x, mut y) = (0, 0);
    for i in 0..n.pow(2) {
        matrix[y as usize][x as usize] = i;
        if x < r {
            x += 1;
        } else {
        }
    }
    matrix
}
mod code_202508;

