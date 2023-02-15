#![allow(dead_code)]

/*

// 1679. Max Number of K-Sum Pairs
// https://leetcode.com/problems/max-number-of-k-sum-pairs/
// https://leetcode.cn/problems/max-number-of-k-sum-pairs/
//
// Medium
//
// You are given an integer array nums and an integer k.

In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.

Return the maximum number of operations you can perform on the array.

Example 1:

Input: nums = [1,2,3,4], k = 5
Output: 2
Explanation: Starting with nums = [1,2,3,4]:
- Remove numbers 1 and 4, then nums = [2,3]
- Remove numbers 2 and 3, then nums = []
There are no more pairs that sum up to 5, hence a total of 2 operations.

Example 2:

Input: nums = [3,1,3,4,3], k = 6
Output: 1
Explanation: Starting with nums = [3,1,3,4,3]:
- Remove the first two 3's, then nums = [1,4,3]
There are no more pairs that sum up to 6, hence a total of 1 operation.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^9
    1 <= k <= 10^9
*/

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut count = 0;
        while i < j {
            let sum = nums[i] + nums[j];
            match sum.cmp(&k) {
                std::cmp::Ordering::Equal => {
                    count += 1;
                    i += 1;
                    j -= 1;
                }
                std::cmp::Ordering::Less => {
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    j -= 1;
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], 5, 2),
        (vec![3, 1, 3, 4, 3], 6, 1),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, 4),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::max_operations(nums, k), expected);
    }
}
