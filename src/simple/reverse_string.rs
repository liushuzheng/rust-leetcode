/// https://leetcode.cn/problems/reverse-string/

pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..len / 2 {
        let tmp = s[i];
        s[i] = s[len - 1 - i];
        s[len - 1 - i] = tmp;
    }
}

#[test]
fn test_reverse_string() {
    let s = &mut vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(s);
    println!("{:?}", s);
}