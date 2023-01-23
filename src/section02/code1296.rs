#![allow(dead_code)]

// 1296. Divide Array in Sets of K Consecutive Numbers
// https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
// https://leetcode.cn/problems/divide-array-in-sets-of-k-consecutive-numbers/
//
// Medium
//
// Given an array of integers nums and a positive integer k, check whether it is possible to divide
// this array into sets of k consecutive numbers.
//
// Return true if it is possible. Otherwise, return false.
//
// Example 1:
//
// Input: nums = [1,2,3,3,4,4,5,6], k = 4
// Output: true
// Explanation: Array can be divided into [1,2,3,4] and [3,4,5,6].
//
// Example 2:
//
// Input: nums = [3,2,1,2,3,4,3,4,5,9,10,11], k = 3
// Output: true
// Explanation: Array can be divided into [1,2,3] , [2,3,4] , [3,4,5] and [9,10,11].
//
// Example 3:
//
// Input: nums = [1,2,3,4], k = 3
// Output: false
// Explanation: Each array should be divided in subarrays of size 3.
//
// Constraints:
//
// -    1 <= k <= nums.length <= 10^5
// -    1 <= nums[i] <= 10^9
//
// Note: This question is the same as 846: https://leetcode.com/problems/hand-of-straights/
//

struct Solution;

impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        while let Some((&n, &count)) = map.iter().next() {
            if count == 0 {
                map.remove(&n);
                continue;
            }
            for i in 0..k {
                let n = n + i;
                if let Some(c) = map.get_mut(&n) {
                    *c -= 1;
                    if *c < 0 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 3, 4, 4, 5, 6], 4, true),
        (vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3, true),
        (vec![1, 2, 3, 4], 3, false),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::is_possible_divide(nums, k), expected);
    }
}
