/*pub fn final_string(s: String) -> String {
    let mut str = "".to_string();
    let chars = s.chars();
    for x in chars {
        if x == 'i' {
            str.chars().rev();
        } else {
            str.push(x);
        }
    }
    return str;
}*/

pub fn final_string(s: String) -> String {
    let mut v = vec![];
    for &x in s.as_bytes().iter() {
        if x == 'i' as u8 {
            v.reverse();
        } else {
            v.push(x);
        }
    }

    return String::from_utf8(v).unwrap();
}

#[test]
fn test_func() {
    let s = "string".to_string();
    let string = final_string(s);
    println!("{}", string)
}