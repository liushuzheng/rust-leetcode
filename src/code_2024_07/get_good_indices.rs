pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    variables.iter()
        .enumerate()
        .filter_map(|(index, v)| {
            if (v[0].pow(v[1] as u32) % 10).pow(v[2] as u32) % v[3] == target {
                Some(index as i32)
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_this() {
    let v = [[2,3,3,10],[3,3,3,1],[6,1,1,4]].iter()
        .map(|row| row.to_vec())
        .collect();
    let result = get_good_indices(v, 2);
    println!("{:?}", result)
}
