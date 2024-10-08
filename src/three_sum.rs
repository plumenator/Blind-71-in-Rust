// https://leetcode.com/problems/3sum

// 15. 3Sum

// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.

// Example 1:

// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.

// Example 2:

// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.

// Example 3:

// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.

// Constraints:

// 3 <= nums.length <= 3000
// -10^5 <= nums[i] <= 10^5

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut threes = HashSet::new();
        for i in 0..nums.len() - 2 {
            let twos = two_sum(&nums[i + 1..nums.len()], -nums[i]);
            for mut two in twos {
                two.push(nums[i]);
                two.sort();
                threes.insert(two);
            }
        }
        threes.into_iter().collect()
    }
}

pub fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        use std::cmp::Ordering::*;
        match (nums[left] + nums[right]).cmp(&target) {
            Less => {
                left += 1;
            }
            Greater => {
                right -= 1;
            }
            Equal => {
                let res = vec![nums[left], nums[right]];
                out.push(res);
                left += 1;
                right -= 1;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new())
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]])
    }
}
