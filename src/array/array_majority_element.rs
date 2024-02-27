use crate::solution::Solution;

impl Solution {

    /**
     * 多数投票算法实现
    **/
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;
        for value in nums {
            if count == 0 {
                candidate = value;
                count = count + 1;
                continue;
            }

            if value == candidate {
                count = count + 1;
            } else {
                count = count -1 ;
            }
        }

        // 前提是已知数组有确定的多数元素，否则就需要再次计数判断取出的多数元素是否是真的多数元素

        candidate
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_majority_element() {
        let nums = vec![3,2,3];
        println!("final value={}", Solution::majority_element(nums));

        let nums = vec![3,3,4];
        println!("final value={}", Solution::majority_element(nums));

        let nums = vec![2,2,1,1,1,2,2];
        println!("final value={}", Solution::majority_element(nums));
    }
}