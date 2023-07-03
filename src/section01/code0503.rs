#![allow(dead_code)]

// 503. Next Greater Element II
// https://leetcode.com/problems/next-greater-element-ii/
// https://leetcode.cn/problems/next-greater-element-ii/
//
// Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]), return the next greater number for every element in nums.
//
// The next greater number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return -1 for this number.
//
// Example 1:
//
// Input: nums = [1,2,1]
// Output: [2,-1,2]
// Explanation: The first 1's next greater number is 2;
// The number 2 can't find next greater number.
// The second 1's next greater number needs to search circularly, which is also 2.
//
// Example 2:
//
// Input: nums = [1,2,3,4,3]
// Output: [2,3,4,-1,4]
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = std::collections::VecDeque::new();
        let mut result = vec![-1; nums.len()];

        for i in 0..nums.len() * 2 {
            let i = i % nums.len();
            while let Some(j) = stack.pop_back() {
                if nums[j] < nums[i] {
                    result[j] = nums[i];
                } else {
                    stack.push_back(j);
                    break;
                }
            }
            stack.push_back(i);
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 3]), vec![2, 3, 4, -1, 4]);
}
