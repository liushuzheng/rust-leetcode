pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    result.push(vec![1]);

    for i in 1..num_rows as usize {
        let mut cow = vec![];
        for j in 0..=i {
            if j == 0 || j == i {
                cow.push(1);
            } else {
                cow.push(result[i - 1][j - 1] + result[i - 1][j]);
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

pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let mut use_baskets = vec![false; baskets.len()];
    fruits.iter().for_each(|x| {
        for (i, v) in baskets.iter().enumerate() {
            if !use_baskets[i] && *v >= *x {
                use_baskets[i] = true;
                break;
            }
        }
    });

    use_baskets.into_iter().filter(|&x| !x).count() as i32
}

#[test]
fn test_num_of_unplaced_fruits() {
    let fruits = vec![4, 2, 5];
    let baskets = vec![3, 5, 4];
    assert_eq!(1, num_of_unplaced_fruits(fruits, baskets));
}

pub fn is_power_of_three(n: i32) -> bool {
    let mut n = n;
    if n < 1 {
        return false;
    }

    while n % 3 == 0 {
        n /= 3;
    }

    n == 1
}

pub fn is_power_of_four(n: i32) -> bool {
    let mut n = n;
    let three = 0b11;
    while n & three == 0 && n != 0 {
        n >>= 2;
    }
    n == 1
}

#[test]
fn test_four(){

    let four = is_power_of_four(16);
    assert!(four);
}
