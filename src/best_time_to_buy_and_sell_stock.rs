pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (_, _, diff) = prices
            .iter()
            .fold((None, None, None), |(buy, sell, diff), &current| {
                match (buy, sell, diff) {
                    (None, None, diff) => (Some(current), None, diff),
                    (Some(buy), None, diff) => {
                        if buy > current {
                            (Some(current), None, diff)
                        } else {
                            (
                                Some(buy),
                                Some(current),
                                if let Some(diff) = diff {
                                    if diff < current - buy {
                                        Some(current - buy)
                                    } else {
                                        Some(diff)
                                    }
                                } else {
                                    Some(current - buy)
                                },
                            )
                        }
                    }
                    (None, Some(sell), diff) => {
                        if sell > current {
                            (None, Some(sell), diff)
                        } else {
                            (Some(sell), Some(current), Some(current - sell))
                        }
                    }
                    (Some(buy), Some(sell), diff) => {
                        if buy > current {
                            (Some(current), None, diff)
                        } else if sell < current {
                            (
                                Some(buy),
                                Some(current),
                                if let Some(diff) = diff {
                                    if diff < current - buy {
                                        Some(current - buy)
                                    } else {
                                        Some(diff)
                                    }
                                } else {
                                    diff
                                },
                            )
                        } else {
                            (Some(buy), Some(sell), diff)
                        }
                    }
                }
            });
        diff.unwrap_or(0)
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
