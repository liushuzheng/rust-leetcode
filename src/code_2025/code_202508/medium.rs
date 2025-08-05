use std::collections::HashSet;

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut max_size = 0;

    let mut left = 0;
    let mut right = 0;
    let mut pre_index = 0;
    let mut set: HashSet<i32> = HashSet::new();

    while left < fruits.len() && right < fruits.len() {
        let i = fruits[right];
        if right + 1 < fruits.len() && fruits[pre_index] == fruits[right + 1] {
            pre_index += 1;
        }
        set.insert(i);
        if set.len() <= 2 {
            right += 1;
            max_size = max_size.max(right - left);
        } else {
            left = pre_index.max(left + 1);
            right = left;
            set.clear();
        }
    }
    max_size as i32
}

#[test]
fn test_total_fruit() {
    let fruits = vec![0, 1, 2, 2];
    assert_eq!(3, total_fruit(fruits));
}
