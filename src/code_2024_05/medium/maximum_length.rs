use std::collections::HashMap;

pub fn maximum_length(s: String) -> i32 {
    let mut map = HashMap::new();
    for x in s.chars() {
        let v = *map.entry(x).or_insert(1);
        if v >= 3 {
            return v;
        }
    }

    -1
}