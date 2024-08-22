pub struct Solution;

// https://leetcode.com/problems/two-sum/description/
// 1. Two Sum
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<_> = nums.iter().copied().zip(0..nums.len()).collect();
        nums.sort_by_key(|(k, _)| *k);
        let (mut left, mut right) = (0, nums.len() - 1);
        loop {
            use std::cmp::Ordering::*;
            match (nums[left].0 + nums[right].0).cmp(&target) {
                Less => {
                    left += 1;
                }
                Greater => {
                    right -= 1;
                }
                Equal => {
                    let mut res = vec![nums[left].1 as i32, nums[right].1 as i32];
                    res.sort();
                    return res;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(super::Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn ex2() {
        assert_eq!(super::Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    fn ex3() {
        assert_eq!(super::Solution::two_sum(vec![3, 3], 6), vec![0, 1])
    }
}
