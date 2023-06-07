//给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，
// 计算按此排列的柱子，下雨之后能接多少雨水。
//输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
// 输出：6
// 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。

pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;
    while left < height.len() - 1 {
        if right >= height.len() - 1 {
            break;
        }
        if height.get(left) > height.get(right) {
            right += 1;
        } else {
            if right - left > 1 {
                println!("left and right is {} ,{} ", left, right);
            }
            left += 1;
            right = left + 1;
        }
    }

    0
}

#[test]
fn test_trap() {
    let v = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    trap(v);
    // assert_eq!(trap(v), 6);
    // let v = vec![4, 2, 0, 3, 2, 5];
    // assert_eq!(trap(v), 9);
}