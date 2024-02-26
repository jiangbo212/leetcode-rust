use std::collections::HashMap;
use crate::solution::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut value_map: HashMap<&i32, i32> = HashMap::new();
        nums.iter().for_each(|value| {
            if value_map.contains_key(value) {
                let _ = &value_map.insert(value, value_map.get(value).unwrap() + 1);
            } else {
                let _ = &value_map.insert(value, 1);
            }
            println!("value={}, value_map={:?}", value, &value_map);
        });

        let half_len: i32 = nums.len() as i32 / 2;

        for (key, value) in &value_map {
            if value > &half_len {
                return *key.to_owned();
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_majority_element() {
        let nums = vec![3,2,3];
        println!("final value={}", Solution::majority_element(nums));

        let nums = vec![2,2,1,1,1,2,2];
        println!("final value={}", Solution::majority_element(nums));
    }
}