pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut v = vec![];
    for x in operations.iter() {
        if x == &"C".to_string() {
            v.pop();
        } else if x == &"D".to_string() {
            v.push(v[v.len() - 1] * 2);
        } else if x == &"+".to_string() {
            v.push(v[v.len() - 1] + v[v.len() - 2]);
        } else {
            v.push(x.parse::<i32>().unwrap());
        }
    }
    
    v.iter().sum()
}