#![allow(dead_code)]

// 456. 132 Pattern
// https://leetcode.com/problems/132-pattern/
//
// Given an array of n integers nums, a 132 pattern is a subsequence of three integers
// nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].
//
// Return true if there is a 132 pattern in nums, otherwise, return false.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: false
// Explanation: There is no 132 pattern in the sequence.
//
// Example 2:
//
// Input: nums = [3,1,4,2]
// Output: true
// Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
//
// Example 3:
//
// Input: nums = [-1,3,2,0]
// Output: true
// Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 2 * 105
// - -109 <= nums[i] <= 109
//

struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut num_j = i32::MIN;
        let mut stack = Vec::new();

        for &num in nums.iter().rev() {
            if num < num_j {
                return true;
            } else {
                loop {
                    match stack.last() {
                        Some(&n) if num > n => {
                            num_j = n;
                            stack.pop();
                        }
                        _ => break,
                    }
                }
                stack.push(num);
            }
        }

        false
    }
}

#[test]
fn test_find132pattern() {
    assert_eq!(Solution::find132pattern(vec![3, 5, 0, 3, 4]), true);
    assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
    assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
    assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
}
