#![allow(dead_code)]

// 260. Single Number III
// https://leetcode.com/problems/single-number-iii/
// https://leetcode.cn/problems/single-number-iii/
//
// Given an integer array nums, in which exactly two elements appear only once and all the other
// elements appear exactly twice. Find the two elements that appear only once.
// You can return the answer in any order.
//
// You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
//
// Example 1:
//
// Input: nums = [1,2,1,3,2,5]
// Output: [3,5]
// Explanation:  [5, 3] is also a valid answer.
//
// Example 2:
//
// Input: nums = [-1,0]
// Output: [-1,0]
//
// Example 3:
//
// Input: nums = [0,1]
// Output: [1,0]
//
// Constraints:
//
// - 2 <= nums.length <= 3 * 10^4
// - -2^31 <= nums[i] <= 2^31 - 1
// - Each integer in nums will appear twice, only two integers will appear once.
//

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        for n in nums.iter() {
            xor ^= n;
        }
        let mut mask = 1;
        while mask & xor == 0 {
            mask <<= 1;
        }
        let mut a = 0;
        let mut b = 0;
        for n in nums.iter() {
            if mask & n == 0 {
                a ^= n;
            } else {
                b ^= n;
            }
        }
        vec![a, b]
    }
}

#[test]
fn test() {
    let mut v = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
    v.sort();
    assert_eq!(v, vec![3, 5]);

    let mut v = Solution::single_number(vec![-1, 0]);
    v.sort();
    assert_eq!(v, vec![-1, 0]);

    let mut v = Solution::single_number(vec![0, 1]);
    v.sort();
    assert_eq!(v, vec![0, 1]);
}
