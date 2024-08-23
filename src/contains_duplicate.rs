// 217. Contains Duplicate

// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

// Example 1:

// Input: nums = [1,2,3,1]
// Output: true
// Example 2:

// Input: nums = [1,2,3,4]
// Output: false
// Example 3:

// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true

// Constraints:

// 1 <= nums.length <= 105
// -109 <= nums[i] <= 109

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::<i32>::new();        
        for num in nums {
          if seen.contains(&num) {
            return true;
          } else {
            seen.insert(num);
          }
        }
      false
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(super::Solution::contains_duplicate(vec![1, 2, 3, 1]), true)
    }

    #[test]
    fn ex2() {
        assert_eq!(super::Solution::contains_duplicate(vec![1, 2, 3, 4]), false)
    }

    #[test]
    fn ex3() {
        assert_eq!(
            super::Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        )
    }
}
