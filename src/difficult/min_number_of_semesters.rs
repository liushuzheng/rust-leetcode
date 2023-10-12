//给你一个整数n表示某所大学里课程的数目，编号为1到n，数组relations中，relations[i] = [xi, yi] 表示一个先修课的关系，也就是课程xi必须在课程yi之前上。同时你还有一个整数k。
// 
// 在一个学期中，你 最多可以同时上 k门课，前提是这些课的先修课在之前的学期里已经上过了。
// 
// 请你返回上完所有课最少需要多少个学期。题目保证一定存在一种上完所有课的方式。
//
// 示例 1：
//
// 输入：n = 4, relations = [[2,1],[3,1],[1,4]], k = 2
// 输出：3 
// 解释：上图展示了题目输入的图。在第一个学期中，我们可以上课程 2 和课程 3 。然后第二个学期上课程 1 ，第三个学期上课程 4 。
//
//
//示例 2：
//
// 输入：n = 5, relations = [[2,1],[3,1],[4,1],[1,5]], k = 2
// 输出：4
// 解释：上图展示了题目输入的图。一个最优方案是：第一学期上课程 2 和 3，第二学期上课程 4 ，第三学期上课程 1 ，第四学期上课程 5 。
// 示例 3：
//
// 输入：n = 11, relations = [], k = 2
// 输出：6


pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {

    0
}

#[test]
fn test_fn(){
  println!("start the method min_number_of_semesters test");
}