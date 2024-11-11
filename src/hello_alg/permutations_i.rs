/* 回溯算法：全排列 I */
use std::collections::HashSet;

fn backtrack(mut state: Vec<i32>, choices: &[i32], selected: &mut [bool], res: &mut Vec<Vec<i32>>) {
    // 当状态长度等于元素数量时，记录解
    if state.len() == choices.len() {
        res.push(state);
        return;
    }
    // 遍历所有选择
    for i in 0..choices.len() {
        let choice = choices[i];
        // 剪枝：不允许重复选择元素
        if !selected[i] {
            // 尝试：做出选择，更新状态
            selected[i] = true;
            state.push(choice);
            // 进行下一轮选择
            backtrack(state.clone(), choices, selected, res);
            // 回退：撤销选择，恢复到之前的状态
            selected[i] = false;
            state.pop();
        }
    }
}

/* 全排列 I */
fn permutations_i(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut res = Vec::new(); // 状态（子集）
    backtrack(Vec::new(), nums, &mut vec![false; nums.len()], &mut res);
    res
}

#[test]
fn test_this_m() {
    let nums = &vec![1, 2, 3][..];
    let vec1 = permutations_i(nums);
    println!("{:?}", vec1)
}


/* 回溯算法：全排列 II */
fn backtrack_ii(mut state: Vec<i32>, choices: &[i32], selected: &mut [bool], res: &mut Vec<Vec<i32>>) {
    // 当状态长度等于元素数量时，记录解
    if state.len() == choices.len() {
        res.push(state);
        return;
    }
    // 遍历所有选择
    let mut duplicated = HashSet::<i32>::new();
    for i in 0..choices.len() {
        let choice = choices[i];
        // 剪枝：不允许重复选择元素 且 不允许重复选择相等元素
        if !selected[i] && !duplicated.contains(&choice) {
            // 尝试：做出选择，更新状态
            duplicated.insert(choice); // 记录选择过的元素值
            selected[i] = true;
            state.push(choice);
            // 进行下一轮选择
            backtrack_ii(state.clone(), choices, selected, res);
            // 回退：撤销选择，恢复到之前的状态
            selected[i] = false;
            state.pop();
        }
    }
}

/* 全排列 II */
fn permutations_ii(nums: &mut [i32]) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    backtrack(Vec::new(), nums, &mut vec![false; nums.len()], &mut res);
    res
}