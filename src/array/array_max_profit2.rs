use crate::solution::Solution;

impl Solution {
    /**
    **  找寻最大利润
    **  只有后一天比前一天价格低，前一天买入，后一天卖出。中间的差价即为利润，将所有利润相加便为最大利润
    **/
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let length = prices.len();

        let mut index: usize = 0;
        let mut profit = 0;
        while index < length - 1 {
            let next_index = index + 1;
            if prices[next_index]  > prices[index] {
                profit = profit + (prices[next_index] - prices[index]);
            }
            index = index + 1;
        }
        profit
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test_max_profit2() {
        let prices = vec![7,1,5,3,6,4];
        assert_eq!(Solution::max_profit2(prices), 7);

        let prices = vec![1,2,3,4,5];
        assert_eq!(Solution::max_profit2(prices), 4);

        let prices =vec![7,6,4,3,1];
        assert_eq!(Solution::max_profit2(prices), 0);
    }
}