
use crate::solution::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        println!("nums1={:?}, m={}, nums2={:?}, n={}", &nums1, m, &nums2,n);
        // 如果n=0，直接返回，因为nums1就是最终结果
        if n == 0 {
            return;
        }

        // 如果m=0，则遍历nums1，将nums2的值均赋值给num1
        if m==0 {
            for i in 0..n as usize {
                nums1[i] = nums2[i]
            }
            return;
        }

        let mut temp_vec = vec![];
        let mut nums2_index: usize = 0;
        let mut nums1_index: usize = 0;
        for i in 0..m as usize {
            // n已经没有元素了
            if nums2_index >= n as usize {
                break;
            }

            nums1_index = i;

            // nums2值小于等于nums1值， 则先添加nums2值，再添加nums1值，并将nums2索引+1
            if nums2[nums2_index] <= nums1[i] {
                temp_vec.push(nums2[nums2_index]);
                loop {
                    nums2_index = nums2_index + 1;
                    if nums2_index >= n as usize ||  nums2[nums2_index] > nums1[i] {
                        break;
                    }
                    temp_vec.push(nums2[nums2_index]);
                }
                temp_vec.push(nums1[i]);
            // nums2值小于nums1值，则先添加nums1值；同时判断nums1的第i+1值是否大于等于num2值，如果是则添加nums2值，同时nums2索引+1; 否则，继续下一个循环继续比较
            } else {
                temp_vec.push(nums1[i]);
                if nums1[i+1] >= nums2[nums2_index] {
                    temp_vec.push(nums2[nums2_index]);
                    nums2_index  = nums2_index + 1;
                }
            }
            println!("num1_index={}, nums2_index={}, temp_vec={:?}", nums1_index, nums2_index, &temp_vec);
        }

        if nums1_index < (m-1) as usize {
            for i in (nums1_index+1)..m as usize {
                temp_vec.push(nums1[i]);
            }
        }

        println!("nums1_index for complete, temp_vec={:?}", temp_vec);

        if nums2_index <= (n -1) as usize {
            for i in nums2_index..n as usize {
                temp_vec.push(nums2[i])
            }
        }

        println!("nums2_index for complete, temp_vec={:?}", temp_vec);

        for i in 0..(m+n) as usize {
            nums1[i] = temp_vec[i]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 = vec![2,5,6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);

        let mut nums1 = vec![1,7,0,0,0];
        let m = 2;
        let mut nums2 = vec![2,5,6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);

        let mut nums1 = vec![1,7,9,0,0];
        let m = 3;
        let mut nums2 = vec![2,5];
        let n = 2;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);

        let mut nums1 = vec![1,0];
        let m = 1;
        let mut nums2 = vec![2];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);

        let mut nums1 = vec![1,2,4,5,6,0];
        let m = 5;
        let mut nums2 = vec![3];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
    }
}
