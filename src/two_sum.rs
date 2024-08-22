pub struct Solution;

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices: HashMap<i32, usize> = HashMap::new();
        for (&n, i) in nums.iter().zip(0..nums.len()) {
            if let Some(&o) = indices.get(&(target - n)) {
                return vec![o as i32, i as i32];
            } else {
                indices.insert(n, i);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(super::Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1])
    }

    #[test]
    fn ex2() {
        assert_eq!(super::Solution::two_sum(vec![3,2,4], 6), vec![1, 2])
    }

    #[test]
    fn ex3() {
        assert_eq!(super::Solution::two_sum(vec![3,3], 6), vec![0, 1])
    }

}