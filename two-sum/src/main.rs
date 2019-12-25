use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 18;

    println!("{:?}", two_sum(nums, target));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut i = 0;

    'outer: for num in &nums {
        'inner: for (k, v) in &map {
            if num == v {
                return vec![*k, i];
            }
        }

        map.insert(i, target - num);
        i += 1;
    }

    vec![]
}
