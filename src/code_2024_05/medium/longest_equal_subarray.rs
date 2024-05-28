use std::cmp;
use std::collections::HashMap;

pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_num = 0;
    let mut map: HashMap<i32, (i32, i32, i32, i32)> = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        map.entry(n)
            .and_modify(|(a, b, c, d)| { (*a += 1, *b, *c = i as i32, *d = i as i32 - *b - *a + 1); })
            .or_insert((1, i as i32, i as i32, 0));
    }


    for x in map.values() {
        if x.3 <= k  {
            max_num = cmp::max(x.0, max_num);
        }
    }
    let aa = 7..6;
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
    
    println!("{:?}", map);
    max_num
}

#[test]
fn test_func() {
    let v = vec![3,1,5,3,1,1];
    let k = 0;
    let max = longest_equal_subarray(v, k);
    println!("{}", max);


}
