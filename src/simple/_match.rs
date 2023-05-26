// //给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
// //
// // '.' 匹配任意单个字符
// // '*' 匹配零个或多个前面的那一个元素
// // 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
// //
// // 输入：s = "aa", p = "a*"
// // 输出：true
// // 解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
// //
//
//
// use std::ops::Index;
//
// pub fn is_match(s: String, p: String) -> bool {
//     let mut s = s.chars();
//     let mut p = p.chars();
//     let mut i1 = 0;
//     let mut i2 = 0;
//     while i2 < p.len() {
//         if let Some(v2) = p.nth(i2) {
//             while i1 < s.len() {
//                 if v2.eq(".") {
//                     i1 += 1;
//                     i2 += 1;
//                     break;
//                 } else if let Some(v) = s.nth(i1) {
//                     if v==v2 {
//                         i1 += 1;
//                         i2 += 1;
//                         break;
//                     } else if v2.eq("*") {
//                         i1 += 1;
//                     } else {
//                         return false;
//                     }
//                 } else {
//                     return false;
//                 }
//             }
//         }
//     }
//     true
// }
//
// #[test]
// fn run_is_match() {
//     let s = String::from("ab");
//     let p = String::from(".*");
//     let is_match = is_match(s, p);
//     println!("is match {}", is_match)
// }