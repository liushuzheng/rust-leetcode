//link https://leetcode.cn/problems/number-of-times-binary-string-is-prefix-aligned/

pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
    if flips.is_empty() {
        return 0;
    }
    let len = flips.len();
    let mut v: Vec<bool> = vec![false; len];
    let mut count: usize = 0;
    let mut high = 0;
    let mut low = 0;

    for x in flips {
        let index = x - 1;
        let value = v.get_mut(index as usize).unwrap();
        *value = true;
        if low >= high {
            count += 1;
        }
        if high < x - 1 {
            high = x - 1;
        }

        let mut flag = true;
        for j in low..high {
            let &z = v.get(j as usize).unwrap();
            if !z {
                flag = false;
                break;
            }
        }
        if flag {
            low = high;
        }
    }
    count as i32
}

#[test]
fn test_fn() {
    let v = vec![3, 2, 4, 1, 5];
    let value = num_times_all_blue(v);
    println!("the result is {}", value);
}