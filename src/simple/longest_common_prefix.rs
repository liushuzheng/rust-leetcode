// 编写一个函数来查找字符串数组中的最长公共前缀。
//
// 如果不存在公共前缀，返回空字符串""。
//
//
//
// 示例 1：
//
// 输入：strs = ["flower","flow","flight"]
// 输出："fl"
// 示例 2：
//
// 输入：strs = ["dog","racecar","car"]
// 输出：""
// 解释：输入不存在公共前缀。

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let first = &strs[0];
    let rest = &strs[1..];

    for (i, ch) in first.char_indices() {
        for str in rest {
            if i >= str.len() || ch != str.chars().nth(i).unwrap() {
                return first[0..i].to_string();
            }
        }
    }

    strs[0].clone()
}

#[test]
fn test_fn() {
    let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let str = longest_common_prefix(strs);
    println!("prefix is {}", str);
}