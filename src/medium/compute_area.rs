//   https://leetcode.cn/problems/rectangle-area/
pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
    let mut two_area = (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1);

    let xb = (ax1 < bx1 && ax2 > bx1) || (bx1 < ax1 && bx2 > ax1);
    let yb = (ay1 < by1 && ay2 > by1) || (by1 < ay1 && by2 > ay1);
    let equal = (ax1 == bx1 && ax2 == bx2) || (ay1 == by1 && ay2 == by2);
    if (xb && yb) || equal {
        let x = [ax1, ax2, bx1, bx2];
        let x = find_second_and_third(x);
        let y = [ay1, ay2, by1, by2];
        let y = find_second_and_third(y);
        two_area -= (x.1 - x.0) * (y.1 - y.0)
    }
    two_area
}


fn find_second_and_third(numbers: [i32; 4]) -> (i32, i32) {
    let mut sorted_numbers = numbers;
    sorted_numbers.sort();  // 对数组进行排序
    let second = sorted_numbers[1];
    let third = sorted_numbers[2];
    (second, third)
}

#[test]
fn test_fn() {
    //-5
    // -3
    // -4
    // 3
    // -3
    // -3
    // 3
    // 3
    let i = compute_area(-5, -3, -4, 3, -3, -3, 3, 3);
    println!("area is {}", i)
}