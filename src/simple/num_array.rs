struct NumArray {
    num: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray {
            num: nums,
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = 0;
        for x in left..=right {
            sum += self.num[x as usize];
        }
        sum
    }
}