pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.into_iter().filter(|&v| v >= target).count() as i32
}