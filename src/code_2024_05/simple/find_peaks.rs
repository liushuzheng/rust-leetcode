pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut v = vec![];
    if mountain.len() <= 2 {
        return v;
    }

    for x in 1..mountain.len() - 1 {
        if mountain[x - 1] < mountain[x] && mountain[x] > mountain[x + 1] {
            v.push(x as i32)
        }
    }
    v
}