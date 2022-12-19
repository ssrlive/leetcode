#![allow(dead_code)]

// 532. K-diff Pairs in an Array
// https://leetcode.com/problems/k-diff-pairs-in-an-array/
// https://leetcode.cn/problems/k-diff-pairs-in-an-array/
//
// Given an array of integers nums and an integer k, return the number of unique k-diff pairs in the array.
//
// A k-diff pair is an integer pair (nums[i], nums[j]), where the following are true:
//
// - 0 <= i, j < nums.length
// - i != j
// - nums[i] - nums[j] == k
//
// Notice that |val| denotes the absolute value of val.
//
// Example 1:
//
// Input: nums = [3,1,4,1,5], k = 2
// Output: 2
// Explanation: There are two 2-diff pairs in the array, (1, 3) and (3, 5).
// Although we have two 1s in the input, we should only return the number of unique pairs.
//
// Example 2:
//
// Input: nums = [1,2,3,4,5], k = 1
// Output: 4
// Explanation: There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).
//
// Example 3:
//
// Input: nums = [1,3,1,5,4], k = 0
// Output: 1
// Explanation: There is one 0-diff pair in the array, (1, 1).
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^7 <= nums[i] <= 10^7
// - 0 <= k <= 10^7
//

struct Solution;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut count = 0;
        for (&n, &c) in map.iter() {
            if k == 0 {
                if c > 1 {
                    count += 1;
                }
            } else if map.contains_key(&(n + k)) {
                count += 1;
            }
        }

        count
    }
}

#[test]
fn test_find_pairs() {
    assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
}
