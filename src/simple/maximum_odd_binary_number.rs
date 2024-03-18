pub fn maximum_odd_binary_number(s: String) -> String {
    let cnt = s.chars().filter(|&c| c == '1').count();
    "1".repeat(cnt - 1) + &*"0".repeat(s.len() - cnt) + "1"
}

#[test]
fn test_this() {
    let string = maximum_odd_binary_number("010".to_string());
    println!("{}", string)
}