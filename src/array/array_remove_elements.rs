use crate::solution::Solution;

impl Solution {
    pub fn remove_elements(nums: &mut Vec<i32>, val: i32) -> i32 {
        let length = nums.len();
        let mut del_element_num = 0;
        for index in 0..length {
            println!("nums={:?}, index={}, del_element_num={}", nums, index, del_element_num);

            // 到达数组末尾
            if index - del_element_num > nums.len() -1 {
                break;
            }

            // 数组删除之后，实际索引位置
            if nums[index-del_element_num] == val {
                nums.remove(index-del_element_num);
                del_element_num = del_element_num + 1;
            }

        }

        println!("final nums={:?}", nums);
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_remove_elements() {
        let mut nums = vec![3,2,2,3];
        let val = 3;
        Solution::remove_elements(&mut nums, val);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        Solution::remove_elements(&mut nums, val);
    }
}