//给你一个下标从 0 开始的字符串数组 words 以及一个二维整数数组 queries 。
//
// 每个查询 queries[i] = [li, ri] 会要求我们统计在 words
// 中下标在 li 到 ri 范围内（包含 这两个值）并且以元音开头和结尾的字符串的数目。
//
// 返回一个整数数组，其中数组的第 i 个元素对应第 i 个查询的答案。
//
// 注意：元音字母是 'a'、'e'、'i'、'o' 和 'u' 。
//
//
// 示例 1：
//
// 输入：words = ["aba","bcb","ece","aa","e"], queries = [[0,2],[1,4],[1,1]]
// 输出：[2,3,0]
// 解释：以元音开头和结尾的字符串是 "aba"、"ece"、"aa" 和 "e" 。
// 查询 [0,2] 结果为 2（字符串 "aba" 和 "ece"）。
// 查询 [1,4] 结果为 3（字符串 "ece"、"aa"、"e"）。
// 查询 [1,1] 结果为 0 。
// 返回结果 [2,3,0] 。

pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let cs = ['a', 'e', 'i', 'o', 'u'];
    let mut v = vec![];
    let result: Vec<bool> = words.iter().map(|x| {
        let mut chars = x.chars();

        let first = chars.next().unwrap();
        match chars.last() {
            Some(last) => {
                return cs.contains(&first) && cs.contains(&last);
            }
            None => {
                return cs.contains(&first);
            }
        }
    }).collect();
    for x in queries.iter() {
        let &first = x.first().unwrap();
        let &last = x.last().unwrap();
        let count = result.iter().enumerate().filter(|(i, &b)| {
            let index = *i as i32;
            return index >= first && index <= last && b;
        }).count();
        v.push(count as i32);
    }
    v
}

#[test]
fn test_fnc() {
    let mut words = vec!["aba".to_string(), "bcb".to_string(), "ece".to_string(), "aa".to_string(), "e".to_string()];
    let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
    let vec = vowel_strings(words, queries);
    assert_eq!(vec![2, 3, 0], vec);
}