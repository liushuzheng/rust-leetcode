pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        let mut index = i + 1;
        loop {
            if index % nums.len() == i {
                v.push(-1);
                break;
            }
            if nums[i] < nums[index % nums.len()] {
                v.push(nums[index % nums.len()]);
                break;
            }
            index += 1;
        }
    }
    v
}

#[test]
fn test_m() {
    let v = vec![1, 2, 1];
    let result = next_greater_elements(v);
    println!("{:?}", result)
}



