pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    if x == 0 {
        return -1;
    }
    let mut next_x = x;
    let mut sum = 0;
    loop {
        sum += next_x % 10;
        next_x = next_x / 10;
        if next_x == 0 {
            break;
        }
    }
    if x % sum == 0 {
        return sum;
    }
    -1
}

#[test]
fn test_fn() {
    let i = sum_of_the_digits_of_harshad_number(18);
    println!("{}", i);
}