pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let mut count = 0;
    for x in 0..words.len() {
        for y in 0..words.len() {
            if x != y {
                let x_str = words.get(x);
                let y_str = words.get(y);
                let y_str_rev: String = y_str.unwrap().chars().rev().collect();
                if x_str.unwrap().eq(&y_str_rev) {
                    count += 1;
                }
            }
        }
    }

    count /2
}

#[test]
fn test_rev() {
    let string = "abc";
    let x: String = string.chars().rev().collect();
    println!("{}", x)
}