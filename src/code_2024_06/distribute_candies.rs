use std::cmp::min;
use std::collections::HashSet;

pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    min(candy_type.iter().collect::<HashSet<_>>().len(), candy_type.len() / 2) as i32
}

pub fn distribute_candies2(candies: i32, num_people: i32) -> Vec<i32> {
    let mut left = candies;
    let mut res = vec![0;num_people as usize];
    let mut start = 0_usize;
    let mut num = 1;
    while left > 0 {
        if left >= num {
            res[start % num_people as usize] += num;
            left -= num;
        } else {
            res[start % num_people as usize] += left;
            left = 0
        }
        start+=1;
        num += 1;
    }
    res
}

#[test]
fn test_2(){
    let vec = distribute_candies2(7, 4);
    println!("{:?}",vec)
}