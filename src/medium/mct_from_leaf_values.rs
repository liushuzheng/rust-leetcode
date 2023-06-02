//给你一个正整数数组 arr，考虑所有满足以下条件的二叉树：
//
// 每个节点都有 0 个或是 2 个子节点。
// 数组 arr 中的值与树的中序遍历中每个叶节点的值一一对应。
// 每个非叶节点的值等于其左子树和右子树中叶节点的最大值的乘积。
// 在所有这样的二叉树中，返回每个非叶节点的值的最小可能总和。这个和的值是一个 32 位整数。
//
// 如果一个节点有 0 个子节点，那么该节点为叶节点。
//
// 输入：arr = [6,2,4]
// 输出：32
// 解释：有两种可能的树，第一种的非叶节点的总和为 36 ，第二种非叶节点的总和为 32 。
pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; n]; n];

    for len in (2..=n).step_by(2) {
        for i in 0..=n - len {
            let j = i + len - 1;
            for k in i + 1..j {
                let mut sum = i32::max(arr[i], arr[j]);
                sum *= dp[i][k] + dp[k + 1][j];
                dp[i][j] = dp[i][j].max(sum);
            }
        }
    }

    dp[0][n - 1]
}


fn min_sum_of_non_leaf_nodes(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut dp = vec![vec![0; n]; n];
    let mut max_val = vec![vec![0; n]; n];

    for i in 0..n {
        max_val[i][i] = arr[i];
        dp[i][i] = 0;
    }

    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            max_val[i][j] = max_val[i][j - 1].max(arr[j]);
            dp[i][j] = i32::MAX;

            for k in i..j {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j] + max_val[i][k] * max_val[k + 1][j]);
            }
        }
    }

    dp[0][n - 1]
}

#[test]
fn test_fn() {
    let arr = vec![6, 2, 4];
    let result = min_sum_of_non_leaf_nodes(&arr);
    println!("Minimum sum of non-leaf nodes: {}", result);
}