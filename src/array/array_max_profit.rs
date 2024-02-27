use crate::solution::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let length = prices.len();
        let mut max_profit = 0;
        for index in 0..length - 1 {
            let compare_index_start = index + 1;
            for compare_index in compare_index_start..length {
                let profit = prices[compare_index] - prices[index];
                if profit > 0 && profit > max_profit {
                    max_profit = profit;
                }
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_max_profit() {
        let prices = vec![7,1,5,3,6,4];
        println!("max profit={}, goal_value={}", Solution::max_profit(prices), 5);

        let prices = vec![7,6,4,3,1];
        println!("max profit={}, goal_value={}", Solution::max_profit(prices), 0)
    }
}