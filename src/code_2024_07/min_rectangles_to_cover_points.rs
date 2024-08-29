use std::collections::HashSet;

pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {

    // 使用迭代器提取每个子向量的第一个元素，去重，并按从大到小排序
    let mut vec: Vec<i32> =
        points.iter()
            .filter_map(|v| v.first().copied())  // 过滤出第一个元素并复制值
            .collect::<HashSet<_>>()              // 去重
            .into_iter()                          // 转换成迭代器
            .collect();                           // 收集到 Vec 中

    // 按从大到小排序
    vec.sort_by(|a, b| a.cmp(b));
    let mut total = 0;
    let mut pre = 0;
    let mut next = 0;
    while pre < vec.len() && next < vec.len() {
        if vec[next] - vec[pre] < w && next == vec.len() - 1 {
            total += 1
        }
        if vec[next] - vec[pre] < w {
            next += 1;
        } else if vec[next] - vec[pre] == w {
            total += 1;
            next += 1;
            pre = next;
        } else {
            total += 1;
            next -= 1;
            pre = next;
        }
    }

    total
}

#[test]
fn test_this() {
    let v = [[1, 3], [7, 3]]
        .iter()
        .map(|row| row.to_vec())
        .collect();

    let result = min_rectangles_to_cover_points(v, 1);

    println!("{}", result)
}