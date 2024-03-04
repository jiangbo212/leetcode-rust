use crate::solution::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        fn jump(nums:&Vec<i32>, jump_start: usize) -> Option<bool> {
            let length = nums.len();
            let can_jump_end = nums[jump_start] as usize;

            // 下一个可jump的位置起始值
            let start = jump_start + 1;
            // 可以到达的元素位置
            let mut end = can_jump_end + jump_start + 1;
            if end > length {
                end = length;
            }

            println!("nums={:?}, length={}, jump_start={}, can_jump_read:{}, start={}, end={}", nums, length, jump_start, can_jump_end, start, end);

            if can_jump_end == 0 && jump_start != length -1 {
                return Some(false);
            }

            if end >= length  {
                return Some(true);
            }

            for index in start..end {
                let jump_result = jump(nums, index);
                if jump_result.unwrap_or(false) == true {
                    return Some(true);
                }
            }

            Some(false)
        }

        let length = nums.len();
        for index in 0..length{
            let jump_result = jump(&nums, index);
            if !jump_result.is_none() {
                return jump_result.unwrap();
            }
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_can_jump() {
        let nums = vec![2,3,1,1,4];
        assert_eq!(Solution::can_jump(nums), true);

        let nums = vec![3,2,1,0,4];
        assert_eq!(Solution::can_jump(nums), false);

        let nums = vec![1];
        assert_eq!(Solution::can_jump(nums), true);

        let nums = vec![1,2,3];
        assert_eq!(Solution::can_jump(nums), true);

        let nums = vec![1,1,1,0];
        assert_eq!(Solution::can_jump(nums), true);
    }
}