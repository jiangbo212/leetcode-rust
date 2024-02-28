use crate::solution::Solution;

impl Solution {
    pub fn roman_to_int(s:String) -> i32 {
        println!("roman_str={}", &s);
        let mut output_int = 0;
        let roman_char = vec!['I', 'V', 'X', 'L', 'C','D', 'M'];
        let roman_int = vec![1, 5, 10, 50, 100, 500, 1000];
        let subtract_mark_char:Vec<usize> = vec![0, 2, 4];
        let roman_chars = s.chars();
        let length = roman_char.len();

        let mut subtract_mark_index = length;
        for c_char in roman_chars {
            for (index, r_char) in roman_char.iter().enumerate() {
                // 获取到匹配字符的索引,跳出索引
                if r_char == &c_char {
                    println!("c_char={}, output_int={}, subtract_mark_index={}, index={}", c_char, output_int, subtract_mark_index, index);
                    // 如果是扣减字符, 且扣减字符索引是初始值, 则将扣减字符的索引写入， 继续下一个循环
                    if subtract_mark_char.contains(&index) && subtract_mark_index == length {
                        subtract_mark_index = index;
                        break;
                    }

                    // 当前索引大于等于扣减索引+2，小于扣减索引, 且小于字符长度 则进入扣减程序。
                    if index <= subtract_mark_index + 2 && index > subtract_mark_index && subtract_mark_index < length {
                        output_int = output_int + (roman_int[index] - roman_int[subtract_mark_index])
                    // 不进入扣减程序则直接加值，如果扣减索引不等于字符长度，则添加扣减索引
                    } else {
                        // 如果扣减索引不是原始值，添加扣减索引值
                        if subtract_mark_index != length {
                            output_int = output_int + roman_int[subtract_mark_index];
                        }

                        // 如果当前值是扣减索引，则扣减索引等于当前索引；否则，添加当前值
                        if subtract_mark_char.contains(&index) {
                            subtract_mark_index = index;
                            break;
                        } else {
                            output_int = output_int + roman_int[index];
                        }
                    }
                    subtract_mark_index = length;
                    break;
                }
            }
        }

        // 如果最后一个字符为扣减值，则直接添加扣减值
        if subtract_mark_index != length {
            output_int = output_int + roman_int[subtract_mark_index];
        }

        println!("output_int={}", output_int);

        output_int
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_roman_to_int() {

        let source_strs = vec!["III", "IV", "IX", "LVIII", "MCMXCIV", "MDCCCLXXXIV", "CCCI"];
        let goal_values = vec![3, 4, 9, 58, 1994, 1884, 301];

        for (index, s) in source_strs.iter().enumerate() {
            let value = Solution::roman_to_int(format!("{}", s));
            assert_eq!(value, goal_values[index])
        }

    }
}