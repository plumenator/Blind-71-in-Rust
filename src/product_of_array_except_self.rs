// 238. Product of Array Except Self

// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

// Example 1:

// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:

// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]

// Constraints:

// 2 <= nums.length <= 10^5
// -30 <= nums[i] <= 30
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let f = |p: &mut _, &n| {
            *p = *p * n;
            Some(*p)
        };
        let prefix = nums.iter().scan(1, f);
        let mut suffix: Vec<_> = nums.iter().rev().scan(1, f).collect();
        suffix.reverse();
        suffix.push(1);
        prefix
            .zip(suffix.iter().skip(1))
            .scan(1, |s, (left, right)| {
                let orig = *s;
                *s = left;
                Some(orig * right)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(
            super::Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            super::Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        )
    }
}
