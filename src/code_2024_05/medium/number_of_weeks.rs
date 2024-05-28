use std::cmp::max;

pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mut longest = milestones[0];
    let mut other:i64 = 0;

    for &m in milestones.iter() {
        longest = max(longest, m);
        other += m as i64;
    }
    other -= longest as i64;

    if longest as i64 > other + 1 {
        other * 2 + 1
    } else {
        other + longest as i64
    }
}