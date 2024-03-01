use crate::solution::Solution;

impl Solution {
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        let mut duplicates_num = 0;
        let mut del_num = 0;
        let mut previous_value = None;
        for index in 0..nums.len() {
            let new_index = index - del_num;
            if new_index>= nums.len() {
                break;
            }
            let value = nums[new_index];
            // 如果previous_value未赋值，则首先进行赋值，同时duplicate+1， 并进入下一个循环
            if previous_value == None {
                previous_value = Some(value);
                duplicates_num = duplicates_num + 1;
            } else {
                if previous_value.unwrap() == value {
                    // 如果duplicate_num小于等于2，继续一个循环
                    if duplicates_num <2 {
                        duplicates_num = duplicates_num + 1;
                    } else {
                        // 元素已经重复两次
                        // nums[index] = temp_value;
                        nums.remove(new_index);
                        del_num = del_num + 1;
                    }
                } else {
                    previous_value = Some(value);
                    duplicates_num = 1;
                }
            }
            println!("new_index={}, previous_value={:?}, value={}, duplicates_num={}, del_num={}, nums={:?}", new_index, previous_value, value, duplicates_num, del_num, nums);

        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_remove_duplicates2() {
        // let mut nums = vec![1,1,1,2,2,3];
        // assert_eq!(Solution::remove_duplicates2(&mut nums), 5);
        //
        // let mut nums = vec![0,0,1,1,1,1,2,3,3];
        // assert_eq!(Solution::remove_duplicates2(&mut nums), 7);

        let mut nums = vec![1,1,1,1];
        assert_eq!(Solution::remove_duplicates2(&mut nums), 2);
    }
}
