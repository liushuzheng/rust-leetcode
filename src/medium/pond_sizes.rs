//https://leetcode.cn/problems/pond-sizes-lcci/

pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
    let v = vec![];

    v
}

#[test]
pub fn test_pond_sizes() {
    let mut matrix = vec![];
    matrix.push(vec![0, 2, 1, 0]);
    matrix.push(vec![0, 1, 0, 1]);
    matrix.push(vec![1, 1, 0, 1]);
    matrix.push(vec![0, 1, 0, 1]);
    let result = pond_sizes(matrix);
    for x in result {
        print!("value is {}", x);
    }
}