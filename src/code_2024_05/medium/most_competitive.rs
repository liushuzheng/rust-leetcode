fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let n = nums.len();
    for i in 0..n {
        while res.len() > 0 && (n - i + res.len()) as i32 > k && *res.last().unwrap() > nums[i] {
            res.pop();
        }
        res.push(nums[i]);
    }
    res.truncate(k as usize);
    res
}

fn test_f(){
    
    let s = "abc".to_string();
   let b= &s[..1];
    
}