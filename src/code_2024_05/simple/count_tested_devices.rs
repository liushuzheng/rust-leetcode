pub fn count_tested_devices(mut battery_percentages: Vec<i32>) -> i32 {

    let mut ptr = 0;
    let mut num = 0;
    while ptr < battery_percentages.len() {
        if battery_percentages[ptr] > 0 {
            num += 1;
            for i in ptr + 1..battery_percentages.len() {
                if battery_percentages[i] > 0 {
                    battery_percentages[i] -= 1
                }
            }
        }
        ptr += 1;
    }

    num
}

#[test]
fn test_func() {
    let v =vec![1,1,2,1,3];
    let i = count_tested_devices(v);
    println!("总共 {i} 步")

}