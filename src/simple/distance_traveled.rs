pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
    let mut used_gas = 0;
    let mut main_gas = main_tank;
    let mut additional_gas = additional_tank;
    while main_gas >= 5 && additional_gas >= 1 {
        used_gas += 5;
        main_gas -= 4;
        additional_gas -= 1;
    }
    used_gas += main_gas;
    used_gas * 10
}


#[test]
fn test_fn() {
    let i = distance_traveled(5, 10);
    println!("total is {}", i)
}


