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
        let mut curr_max = nums[0];
        let mut so_far = nums[0];
        for num in nums.iter().skip(1) {
            so_far = if so_far < 0 { *num } else { so_far + *num };
            curr_max = curr_max.max(so_far);
        }
        curr_max
    }
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
