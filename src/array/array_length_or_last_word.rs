use crate::solution::Solution;

impl Solution {
    pub fn length_of_last_word(s:String) -> i32 {
        let mut length_of_last_world = 0;
        // 字符前面出现过空格
        let mut space_mark = false;
        for c_value in s.chars(){
            if c_value == ' ' {
                space_mark = true;
            } else {
                if space_mark == true {
                    length_of_last_world = 0;
                    space_mark =false;
                }
                length_of_last_world = length_of_last_world + 1;
            }
            // println!("c_value={}, space_mark={}, length_of_last_word={}", c_value, space_mark, length_of_last_world);
        }
        length_of_last_world
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_last_of_last_word() {
        let s = String::from("Hello World");
        println!("length_of_last_word={}", Solution::length_of_last_word(s));

        let s = String::from("Hello World ");
        println!("length_of_last_word={}", Solution::length_of_last_word(s));
    }
}