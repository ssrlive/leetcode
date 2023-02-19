#![allow(dead_code)]

/*

// 1748. Sum of Unique Elements
// https://leetcode.com/problems/sum-of-unique-elements/
// https://leetcode.cn/problems/sum-of-unique-elements/
//
// Easy
//
// You are given an integer array nums. The unique elements of an array are the elements that appear exactly once in the array.

Return the sum of all the unique elements of nums.

Example 1:

Input: nums = [1,2,3,2]
Output: 4
Explanation: The unique elements are [1,3], and the sum is 4.

Example 2:

Input: nums = [1,1,1,1,1]
Output: 0
Explanation: There are no unique elements, and the sum is 0.

Example 3:

Input: nums = [1,2,3,4,5]
Output: 15
Explanation: The unique elements are [1,2,3,4,5], and the sum is 15.

Constraints:

    1 <= nums.length <= 100
    1 <= nums[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];
        for &num in nums.iter() {
            count[num as usize] += 1;
        }
        let mut sum = 0;
        for (i, &c) in count.iter().enumerate() {
            if c == 1 {
                sum += i;
            }
        }
        sum as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
    assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}
