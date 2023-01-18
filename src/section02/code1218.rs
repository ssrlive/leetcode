#![allow(dead_code)]

// 1218. Longest Arithmetic Subsequence of Given Difference
// https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/
// https://leetcode.cn/problems/longest-arithmetic-subsequence-of-given-difference/
//
// Medium
//
// Given an integer array arr and an integer difference, return the length of the longest subsequence in arr which is an arithmetic sequence such that the difference between adjacent elements in the subsequence equals difference.
//
// A subsequence is a sequence that can be derived from arr by deleting some or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: arr = [1,2,3,4], difference = 1
// Output: 4
// Explanation: The longest arithmetic subsequence is [1,2,3,4].
//
// Example 2:
//
// Input: arr = [1,3,5,7], difference = 1
// Output: 1
// Explanation: The longest arithmetic subsequence is any single element.
//
// Example 3:
//
// Input: arr = [1,5,7,8,5,3,4,2,1], difference = -2
// Output: 4
// Explanation: The longest arithmetic subsequence is [7,5,3,1].
//
// Constraints:
//
// -    1 <= arr.length <= 10^5
// -    -10^4 <= arr[i], difference <= 10^4
//

struct Solution;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        use std::collections::HashMap;
        let mut mp = HashMap::new();
        let mut ans = 0;
        for x in arr {
            let y = mp.get(&(x - difference)).unwrap_or(&0) + 1;
            mp.insert(x, y);
            ans = ans.max(y);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], 1, 4),
        (vec![1, 3, 5, 7], 1, 1),
        (vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2, 4),
    ];
    for (arr, difference, expected) in cases {
        assert_eq!(Solution::longest_subsequence(arr, difference), expected);
    }
}
