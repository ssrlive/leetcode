#![allow(dead_code)]

// 1262. Greatest Sum Divisible by Three
// https://leetcode.com/problems/greatest-sum-divisible-by-three/
// https://leetcode.cn/problems/greatest-sum-divisible-by-three/
//
// Medium
//
// Given an integer array nums, return the maximum possible sum of elements of the array such that it is divisible by three.
//
// Example 1:
//
// Input: nums = [3,6,5,1,8]
// Output: 18
// Explanation: Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).
//
// Example 2:
//
// Input: nums = [4]
// Output: 0
// Explanation: Since 4 is not divisible by 3, do not pick any number.
//
// Example 3:
//
// Input: nums = [1,2,3,4,4]
// Output: 12
// Explanation: Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
//
// Constraints:
//
// -    1 <= nums.length <= 4 * 10^4
// -    1 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ones = vec![];
        let mut twos = vec![];

        let mut tot = 0;
        for v in nums {
            let rv = v % 3;
            if rv == 1 {
                if ones.len() < 2 {
                    ones.push(v);
                }
            } else if rv == 2 && twos.len() < 2 {
                twos.push(v);
            }
            tot += v;
        }

        let rv = tot % 3;
        if rv == 0 {
            tot
        } else if rv == 1 {
            let mut result = tot - ones[0];
            if twos.len() > 1 {
                result = std::cmp::max(result, tot - twos[0] - twos[1]);
            }
            result
        } else {
            let mut result = 0;
            if !twos.is_empty() {
                result = tot - twos[0];
            }
            if ones.len() > 1 {
                result = std::cmp::max(result, tot - ones[0] - ones[1]);
            }
            result
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
    assert_eq!(Solution::max_sum_div_three(vec![4]), 0);
    assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
}
