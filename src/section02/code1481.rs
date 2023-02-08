#![allow(dead_code)]

/*

// 1481. Least Number of Unique Integers after K Removals
// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
// https://leetcode.cn/problems/least-number-of-unique-integers-after-k-removals/
//
// Medium
//
// Given an array of integers arr and an integer k. Find the least number of unique integers after removing exactly k elements.
//
// Example 1:

Input: arr = [5,5,4], k = 1
Output: 1
Explanation: Remove the single 4, only 5 is left.

Example 2:

Input: arr = [4,3,1,1,3,3,2], k = 3
Output: 2
Explanation: Remove 4, 2 and either one of the two 1s or three 3s. 1 and 3 will be left.

Constraints:

    1 <= arr.length <= 10^5
    1 <= arr[i] <= 10^9
    0 <= k <= arr.length
*/

struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in arr {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut v = map.values().collect::<Vec<_>>();
        v.sort();
        let mut k = k;
        let mut i = 0;
        while k > 0 {
            k -= v[i];
            i += 1;
        }
        if k < 0 {
            i -= 1;
        }
        (v.len() - i) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![5, 5, 4], 1, 1),
        (vec![4, 3, 1, 1, 3, 3, 2], 3, 2),
        (vec![1, 1, 2, 3, 3, 3, 4, 4, 4, 4], 3, 2),
    ];
    for (arr, k, expected) in cases {
        assert_eq!(Solution::find_least_num_of_unique_ints(arr, k), expected);
    }
}
