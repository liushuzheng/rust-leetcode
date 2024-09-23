pub fn clear_digits(s: String) -> String {
    let res = s.as_str().chars().fold(Vec::<char>::new(), |mut res, c| {
        if c.is_digit(10) {
            res.pop();
        } else {
            res.push(c);
        }
        return res;
    });
    res.into_iter().collect()
}