// https://leetcode.com/problems/maximum-subarray/
// 53. Maximum Subarray
// Given an integer array nums, find the
// subarray
//  with the largest sum, and return its sum.

// Example 1:

// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: The subarray [4,-1,2,1] has the largest sum 6.

// Example 2:

// Input: nums = [1]
// Output: 1
// Explanation: The subarray [1] has the largest sum 1.

// Example 3:

// Input: nums = [5,4,-1,7,8]
// Output: 23
// Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.

// Constraints:

// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4

// Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        max_sub_array_rec(&nums)
    }
}

fn max_sub_array_rec(
    nums: &[i32],
) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let (left, right) = nums.split_at(nums.len() / 2);
        let lmax = max_sub_array_rec(left);
        let rmax = max_sub_array_rec(right);
        let mmax = mid_max(left, right);
        lmax.max(rmax).max(mmax)
    }
}

fn mid_max(left: &[i32], right: &[i32]) -> i32 {
        let f = |(sum, max), &n| {
            let update = sum + n;
            if update > max {
                (update, update)
            } else {
                (update, max)
            }
        };
        let (_, lmax) = left.iter().rev().fold((0, i32::min_value()), f);
        let (_, rmax) = right.iter().fold((0, i32::min_value()), f);
        lmax + rmax
}

#[cfg(test)]
mod tests {
    #[test]
    fn ex1() {
        assert_eq!(
            super::Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        )
    }
    #[test]
    fn ex2() {
        assert_eq!(super::Solution::max_sub_array(vec![1]), 1)
    }
    #[test]
    fn ex3() {
        assert_eq!(super::Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23)
    }
}
