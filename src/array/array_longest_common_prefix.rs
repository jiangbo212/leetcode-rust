use crate::solution::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let length = strs.len();

        let mut char_index: usize = 0;
        let mut common_prefixs = vec![];
        let mut compare_char = None ;

        if strs.len() == 1 {
            return format!("{}", strs[0]);
        }

        loop {
            for str_index in 0..length {
                let str_chars = &strs[str_index].chars().collect::<Vec<char>>();
                // 比较字符索引大于任一字符长度，直接退出循环
                if char_index >= str_chars.len() {
                    return String::from_utf8(common_prefixs).unwrap();
                }
                let str_char = str_chars[char_index];
                println!("str={:?}, str_index={}, char_index={}, str_char={}", str_chars, str_index, char_index, str_char);
                // 待比较字符是None或是待比较的第一个字符串，给待比较字符赋值; 否则，进行比较
                if compare_char == None || str_index ==0 {
                    compare_char = Some(str_char);
                } else {
                    // 如果待比较字符与当前字符一致，
                    if compare_char.unwrap() == str_char  {
                        //且到达最后一个字符串，添加一致字符, 否则继续下一个循环
                        if str_index == length -1 {
                            common_prefixs.push(str_char as u8);
                            char_index = char_index + 1;
                        } else {
                            continue;
                        }
                    } else {
                        return String::from_utf8(common_prefixs).unwrap();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_longest_common_fix() {
        let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
        assert_eq!(Solution::longest_common_prefix(strs), "fl");

        let strs = vec![String::from("dog"), String::from("racecar"), String::from("car")];
        assert_eq!(Solution::longest_common_prefix(strs), "");

        let strs = vec![String::from("a")];
        assert_eq!(Solution::longest_common_prefix(strs), "a");
    }
}