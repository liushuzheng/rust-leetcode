const G: [char; 3] = ['M', 'P', 'G'];

pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    G.iter().fold(0, |acc, &e| acc + one_garbage_time(e, &garbage, &travel))
}

pub fn one_garbage_time(ch: char, garbage: &Vec<String>, travel: &Vec<i32>) -> i32 {
    let mut time = 0;
    let mut step: i32 = -1;
    for index in 0..garbage.len() {
        let s = garbage.get(index).unwrap();
        let count = s.chars().filter(|&c| c == ch).count();
        time += count;
        if count > 0 {
            step = index as i32 - 1;
        }
    }
    if step >= 0 {
        time += (0..=step)
            .fold(0, |acc, e| acc as i32 + travel[e as usize]) as usize;
    }
    time as i32
}


#[test]
fn test_func() {
    let garbage = vec!["G".to_string(), "P".to_string(), "GP".to_string(), "GG".to_string()];
    let travel = vec![2, 4, 3];
    let time = garbage_collection(garbage, travel);
    println!("{}", time);
}

