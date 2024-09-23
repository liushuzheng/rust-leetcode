pub fn remove_stars(s: String) -> String {
    s.chars().into_iter().fold(Vec::new(), |mut v, x| {
        if x != '*' {
            v.push(x);
            v
        } else {
            v.pop();
            v
        }
    }).into_iter().collect()
}