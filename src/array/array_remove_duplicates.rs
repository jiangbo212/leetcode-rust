use crate::solution::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let length =nums.len();
        let mut del_element_num = 0;
        for index in 0..length {
            println!("index={}, del_element_num={}, new_nums_length={}, nums={:?}", index, del_element_num, nums.len(), nums);

            let new_index = index - del_element_num;

            // 如果删除后的nums的新索引位置大约删除后的nums的长度
            if  new_index >= nums.len() - 1 {
                break;
            }

            if nums[new_index] == nums[new_index + 1] {
                nums.remove(new_index);
                del_element_num = del_element_num + 1;
            }
        }

        println!("final nums= {:?}", nums);

        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_remove_duplicates() {
        let mut  nums = vec![1,1,2];
        Solution::remove_duplicates(&mut nums);

        let mut nums= vec![0,0, 1,1,1,2,2,3,3,4];
        Solution::remove_duplicates(&mut nums);
    }
}