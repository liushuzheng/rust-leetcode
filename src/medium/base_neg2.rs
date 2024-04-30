pub fn base_neg2(n: i32) -> String {
    let mut result = String::new();
    if n == 0 {
        result.push_str("0");
        return result;
    }
    let mv = -2;
    let mut n = n;

    while n != 0 {
        let mut mod_v = n % mv;
        n = n / mv;
        if mod_v == -1 {
            n += 1;
            mod_v = 1
        }
        result.push_str(mod_v.to_string().as_str())
    }
    result.chars().rev().collect::<String>()
}

#[test]
fn test_base_neg2() {
    let string = base_neg2(6);
    println!("{string}")
}