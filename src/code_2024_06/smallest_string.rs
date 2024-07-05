pub fn smallest_string(s: String) -> String {

    let  v: Vec<usize> = s.chars()
        .enumerate()
        .filter_map(|(u, c)| if c == 'a' { Some(u) } else { None })
        .collect();
    if v.is_empty() {
        return s.chars().map(|c| (c as u8 + 1) as char).collect();
    }
   
    let option = v.windows(2).map(|w| w[1] - w[0]).min();

    s
}