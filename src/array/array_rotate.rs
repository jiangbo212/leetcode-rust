use crate::solution::Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k:i32) {
        let num_length = nums.len();
        let mut new_nums = vec![0;num_length];

        let mut new_k = k as usize % num_length;
        let value = k as usize / num_length;
        println!("nums={:?}, new_k={}, value={}", nums, new_k, value);

        if new_k != 0 {
            if value / 2 == 1 {
                new_k = num_length - new_k;
            }

            for index in 0..num_length {
                let mut num_index = num_length - new_k + index;
                if index >= new_k {
                    num_index = index - new_k;
                }
                new_nums[index] = nums[num_index];
            }

            for index in 0..num_length {
                nums[index] = new_nums[index];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1,2,3,4,5,6,7];
        let k = 3;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![5,6,7,1,2,3,4]);

        let mut nums = vec![-1,-100,3,99];
        let k = 2;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![3,99,-1,-100]);

        let mut nums = vec![-1];
        let k = 2;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![-1]);

        let mut nums = vec![1,2];
        let k = 3;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![2, 1]);

        let mut nums = vec![1,2];
        let k = 2;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![1, 2]);

        let mut nums = vec![1,2,3];
        let k = 4;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![3, 1, 2]);
    }
}