/* 回溯算法：子集和 I */
fn backtrack(
    state: &mut Vec<i32>,
    target: i32,
    total: i32,
    choices: &[i32],
    res: &mut Vec<Vec<i32>>,
) {
    // 子集和等于 target 时，记录解
    if total == target {
        res.push(state.clone());
        return;
    }
    // 遍历所有选择
    for i in 0..choices.len() {
        // 剪枝：若子集和超过 target ，则跳过该选择
        if total + choices[i] > target {
            continue;
        }
        // 尝试：做出选择，更新元素和 total
        state.push(choices[i]);
        // 进行下一轮选择
        backtrack(state, target, total + choices[i], choices, res);
        // 回退：撤销选择，恢复到之前的状态
        state.pop();
    }
}

/* 求解子集和 I（包含重复子集） */
fn subset_sum_i_naive(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut state = Vec::new(); // 状态（子集）
    let total = 0; // 子集和
    let mut res = Vec::new(); // 结果列表（子集列表）
    backtrack(&mut state, target, total, nums, &mut res);
    res
}


/* 回溯算法：子集和 II */
fn backtrack_i(
    state: &mut Vec<i32>,
    target: i32,
    choices: &[i32],
    start: usize,
    res: &mut Vec<Vec<i32>>,
) {
    // 子集和等于 target 时，记录解
    if target == 0 {
        res.push(state.clone());
        return;
    }
    // 遍历所有选择
    // 剪枝二：从 start 开始遍历，避免生成重复子集
    // 剪枝三：从 start 开始遍历，避免重复选择同一元素
    for i in start..choices.len() {
        // 剪枝一：若子集和超过 target ，则直接结束循环
        // 这是因为数组已排序，后边元素更大，子集和一定超过 target
        if target - choices[i] < 0 {
            break;
        }
        // 剪枝四：如果该元素与左边元素相等，说明该搜索分支重复，直接跳过
        if i > start && choices[i] == choices[i - 1] {
            continue;
        }
        // 尝试：做出选择，更新 target, start
        state.push(choices[i]);
        // 进行下一轮选择
        backtrack_i(state, target - choices[i], choices, i + 1, res);
        // 回退：撤销选择，恢复到之前的状态
        state.pop();
    }
}

/* 求解子集和 II */
fn subset_sum_ii(nums: &mut [i32], target: i32) -> Vec<Vec<i32>> {
    let mut state = Vec::new(); // 状态（子集）
    nums.sort(); // 对 nums 进行排序
    let start = 0; // 遍历起始点
    let mut res = Vec::new(); // 结果列表（子集列表）
    backtrack_i(&mut state, target, nums, start, &mut res);
    res
}