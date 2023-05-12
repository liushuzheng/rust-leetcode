//给定两个大小分别为 m 和 n 的正序（从小到大）数组nums1 和nums2。
// 请你找出并返回这两个正序数组的 中位数 。
// 算法的时间复杂度应该为 O(log (m+n)) 。

//输入：nums1 = [1,2], nums2 = [3,4]
// 输出：2.50000
// 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len = nums1.len() + nums2.len();
    let m1 = len / 2;
    let m2 = (len + 1) / 2;
    let result = mid(nums1, nums2, m1, m2);

    println!("{} {}", result.0, result.1);
    ((result.0 + result.1) / 2) as f64
}

pub fn mid(nums1: Vec<i32>, nums2: Vec<i32>, mid: usize, m1: usize) -> (i32, i32) {
    let mut in1: usize = 0;
    let mut in2: usize = 0;

    let mut min: i32 = 0;

    while (in1 + in2) < mid {
        if nums1.get(in1) <= nums2.get(in2) {
            if let Some(v) = nums1.get(in1) {
                min = *v;
                in1 += 1;
            }
        } else {
            if let Some(v) = nums2.get(in2) {
                min = *v;
                in2 += 1;
            }
        }

        print!("in1 {} in2 {} mid {}  ", in1, in2, mid);
    }
    if mid != m1 {
        return (min, min);
    }
    let left = min;

    if nums1.get(in1) <= nums2.get(in2) {
        if let Some(v) = nums1.get(in1) {
            min = *v;
            // in1 += 1;
        }
    } else {
        if let Some(v) = nums2.get(in2) {
            min = *v;
            // in2 += 1;
        }
    }

    return (left, min);
}

#[test]
fn test_t() {
    let num = find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
    println!("{}", num)
}