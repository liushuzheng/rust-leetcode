// 在一个由 '0' 和 '1' 组成的二维矩阵内，找到只包含 '1' 的最大正方形，并返回其面积。
// 输入：matrix = [["1","0","1","0","0"],["1","0","1","1","1"],
// ["1","1","1","1","1"],["1","0","0","1","0"]]
// 输出：4

use std::fs::read;

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut len = 1;
    let bound = matrix.len();
    for i in 0..bound {
        let x = matrix.get(i).unwrap();
        for j in 0..x.len() {
            let &y = x.get(j).unwrap();
            if y == '0' {
                continue;
            }
            while area(&matrix, i, j, len) {
                len += 1;
            }
        }
    }

    len -= 1;

    (len * len) as i32
}

fn area(matrix: &Vec<Vec<char>>, x: usize, y: usize, len: usize) -> bool {
    if x + len > matrix.len() {
        return false;
    }
    if y + len > matrix.get(0).unwrap().len() {
        return false;
    }

    for i in x..x + len {
        let xv = matrix.get(i).unwrap();
        for j in y..y + len {
            let yv = xv.get(j).unwrap();
            if *yv == '0' {
                return false;
            }
        }
    }

    true
}

#[test]
fn test_func() {
    let matrix = [['1', '0', '1', '0', '0'], ['1', '0', '1', '1', '1'],
        ['1', '1', '1', '1', '1'], ['1', '0', '0', '1', '0']];

    // let matrix = [['1', '1'], ['1', '1']];
    let m: Vec<Vec<char>> = matrix.iter()
        .map(|&x| x.into_iter().collect())
        .collect();
    let i = maximal_square(m);
    // let i = area(&m, 0, 0, 2);
    println!("max square is {}", i);
}
