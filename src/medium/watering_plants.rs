pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    let mut step = 0;
    let mut use_cap = capacity;
    let mut visited = 0;

    while visited < plants.len() {
        use_cap = use_cap - plants[visited];

        if use_cap >= 0 {
            visited += 1;
        } else {
            use_cap = capacity;
            step += visited * 2;
        }
    }

    (step + plants.len()) as i32
}

#[test]
fn test_fn() {
    let v = vec![7,7,7,7,7,7,7];
    let capacity = 8;

    let i = watering_plants(v, capacity);
    println!("{}", i);
}