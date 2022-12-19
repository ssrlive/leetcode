#![allow(dead_code)]

// 347. Top K Frequent Elements
// https://leetcode.com/problems/top-k-frequent-elements/
// https://leetcode.cn/problems/top-k-frequent-elements/
//
// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
//
// Example 1:
//
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]
//
// Example 2:
//
// Input: nums = [1], k = 1
// Output: [1]
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - k is in the range [1, the number of unique elements in the array].
// - It is guaranteed that the answer is unique.
//
// Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
//

struct Solution;

// use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for n in nums {
            let val = map.entry(n).or_insert(0);
            *val += 1;
        }
        let mut res: Vec<_> = map.iter_mut().collect();
        res.sort_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap());
        res.into_iter().rev().take(k as usize).map(|(k, _)| *k).collect()
    }
}

#[test]
fn test_top_k_frequent() {
    assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
