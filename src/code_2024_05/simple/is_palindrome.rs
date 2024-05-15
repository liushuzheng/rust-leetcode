pub fn is_palindrome(x: i32) -> bool {
    let str = x.to_string();
    let u8s = str.as_bytes();
    let len = u8s.len();
    for i in 0..len / 2 {
        if !u8s[i].eq(&u8s[len -1 - i]) {
            return false;
        }
    }
    true
}