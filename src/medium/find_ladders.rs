//给定字典中的两个词，长度相等。写一个方法，把一个词转换成另一个词， 但是一次只能改变一个字符。每一步得到的新词都必须能在字典中找到。
// 编写一个程序，返回一个可能的转换序列。如有多个可能的转换序列，你可以返回任何一个。
// 示例 1:
// 输入:
// beginWord = "hit",
// endWord = "cog",
// wordList = ["hot","dot","dog","lot","log","cog"]
// 输出:
// ["hit","hot","dot","lot","log","cog"]

// 示例 2:
// 输入:
// beginWord = "hit"
// endWord = "cog"
// wordList = ["hot","dot","dog","lot","log"]
// 输出: []
// 解释:endWord "cog" 不在字典中，所以不存在符合要求的转换序列。

pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<String> {
    let v = vec![];
    v
}

#[test]
fn test_fn() {
    let beginWord = "hit".to_string();
    let endWord = "cog".to_string();
    let wordList = vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()];
    let v = find_ladders(beginWord, endWord, wordList);
    vec1
}