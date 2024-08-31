// https://leetcode.com/problems/search-in-rotated-sorted-array

// 33. Search in Rotated Sorted Array

// There is an integer array nums sorted in ascending order (with distinct values).

// Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

// You must write an algorithm with O(log n) runtime complexity.

// Example 1:

// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4

// Example 2:

// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1

// Example 3:

// Input: nums = [1], target = 0

// Output: -1

// Constraints:

// 1 <= nums.length <= 5000
// -10^4 <= nums[i] <= 10^4
// All values of nums are unique.
// nums is an ascending array that is possibly rotated.
// -10^4 <= target <= 10^4

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut begin = 0;
        let mut end = nums.len() - 1;
        while begin != end {
            let mid = (begin + end) / 2;
            if (target >= nums[0]) == (nums[mid] >= nums[0]) {
                if target > nums[mid] {
                    begin = mid + 1
                } else {
                    end = mid;
                }
            } else if target < nums[0] {
                begin = mid + 1;
            } else {
                end = mid;
            }
        }
        if nums[begin] == target {
            begin as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1)
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::search(vec![1], 0), -1)
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::search(vec![1, 3], 1), 0)
    }

    #[test]
    fn ex5() {
        assert_eq!(Solution::search(vec![1, 3], 3), 1)
    }

    #[test]
    fn ex6() {
        assert_eq!(Solution::search(vec![5, 1, 2, 3, 4], 1), 1)
    }
}
