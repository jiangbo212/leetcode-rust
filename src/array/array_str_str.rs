use crate::solution::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_chars = needle.chars().collect::<Vec<char>>();
        let haystack_chars = haystack.chars().collect::<Vec<char>>();
        let haystack_length = haystack_chars.len();
        let needle_length = needle_chars.len();
        if haystack_length < needle_length {
            return -1;
        }

        for haystack_index in 0..haystack_length - needle_length + 1 {
            for needle_index in 0.. needle_length {
                let compare_haystack_index = haystack_index + needle_index;
                // 如果待比较的haystack的索引大于haystack长度，就可以退出needle的循环了
                if compare_haystack_index == haystack_length {
                    break;
                }

                // 如果字符比较不一致，则继续循环haystack
                let compare_haystack_char = haystack_chars[compare_haystack_index];
                if compare_haystack_char != needle_chars[needle_index] {
                    break;
                }

                if needle_index == needle_length -1 {
                    return haystack_index as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_str_str() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let str_str_value = Solution::str_str(haystack.clone(), needle.clone());
        println!("haystack={}, needle={}, final str_str_value: {}", haystack, needle,str_str_value );
        assert_eq!(str_str_value, 0);

        let haystack = String::from("aadbutsad");
        let needle = String::from("sad");
        let str_str_value = Solution::str_str(haystack.clone(), needle.clone());
        println!("haystack={}, needle={}, final str_str_value: {}", haystack, needle,str_str_value );
        assert_eq!(str_str_value, 6);

        let haystack = String::from("aadbutsad");
        let needle = String::from("but");
        let str_str_value = Solution::str_str(haystack.clone(), needle.clone());
        println!("haystack={}, needle={}, final str_str_value: {}", haystack, needle,str_str_value );
        assert_eq!(str_str_value, 3);

        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let str_str_value = Solution::str_str(haystack.clone(), needle.clone());
        println!("haystack={}, needle={}, final str_str_value: {}", haystack, needle,str_str_value );
        assert_eq!(str_str_value, -1);

        let haystack = String::from("mississippi");
        let needle = String::from("issip");
        let str_str_value = Solution::str_str(haystack.clone(), needle.clone());
        println!("haystack={}, needle={}, final str_str_value: {}", haystack, needle,str_str_value );
        assert_eq!(str_str_value, 4);

        let haystack = String::from("sad");
        let needle = String::from("sad");
        let str_str_value = Solution::str_str(haystack.clone(), needle.clone());
        println!("haystack={}, needle={}, final str_str_value: {}", haystack, needle,str_str_value );
        assert_eq!(str_str_value, 0);
    }
}