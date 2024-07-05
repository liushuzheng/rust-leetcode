use std::cmp;

pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v = vec![];
    if matrix.is_empty() {
        return v;
    }
    let m = matrix.len() as usize;
    let n = matrix[0].len() as usize;
    for x in matrix {
        v.push(x);
    }
    for i in 0..n {
        let mut max_value = -1;
        for j in 0..m {
            max_value = cmp::max(max_value, v[j][i])
        }

        for j in 0..m {
            if v[j][i] == -1 {
                v[j][i] = max_value;
            }
        }
    }
    v
}

#[test]
fn test_this(){
  let v=  vec![vec![1,2,-1],vec![4,-1,6],vec![7,8,9]];
    let vec1 = modified_matrix(v);
    println!("{:?}",vec1)
    

}