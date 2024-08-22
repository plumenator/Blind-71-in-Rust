pub struct Solution;

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
// 121. Best Time to Buy and Sell Stock
// You are given an array prices where prices[i] is the price of a given stock on the ith day.

// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

// Example 1:

// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
// Example 2:

// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are done and the max profit = 0.

// Constraints:

// 1 <= prices.length <= 105
// 0 <= prices[i] <= 104

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (_, diff) = prices
            .iter()
            .skip(1)
            .fold((prices[0], 0), |(buy, diff), &current| {
                if buy > current {
                    (current, diff)
                } else {
                    (
                        buy,
                        if diff < current - buy {
                            current - buy
                        } else {
                            diff
                        },
                    )
                }
            });
        diff.max(0)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(super::Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
    }

    #[test]
    fn ex2() {
        assert_eq!(super::Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
    }

    #[test]
    fn ex3() {
        assert_eq!(super::Solution::max_profit(vec![2, 4, 1]), 2)
    }

    #[test]
    fn ex4() {
        assert_eq!(super::Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 4)
    }
}
