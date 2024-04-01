use std::collections::HashMap;
use std::iter::Map;

struct FrequencyTracker {
    member_map: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            member_map: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        match self.member_map.get(&number) {
            None => {
                self.member_map.insert(number, 1);
            }
            Some(count) => {
                self.member_map.insert(number, count + 1);
            }
        }
    }

    fn delete_one(&mut self, number: i32) {
        match self.member_map.get(&number) {
            Some(1) => {
                self.member_map.remove(&number);
            }
            Some(count) => {
                self.member_map.insert(number, count - 1);
            }
            _ => {}
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.member_map.values().any(|&x| x == frequency)
    }
}
