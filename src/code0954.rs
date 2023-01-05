#![allow(dead_code)]

// 954. Array of Doubled Pairs
// https://leetcode.com/problems/array-of-doubled-pairs/
// https://leetcode.cn/problems/array-of-doubled-pairs/
//
// Given an integer array of even length arr, return true if it is possible to reorder arr such
// that arr[2 * i + 1] = 2 * arr[2 * i] for every 0 <= i < len(arr) / 2, or false otherwise.
//
// Example 1:
//
// Input: arr = [3,1,3,6]
// Output: false
//
// Example 2:
//
// Input: arr = [2,1,2,6]
// Output: false
//
// Example 3:
//
// Input: arr = [4,-2,2,-4]
// Output: true
// Explanation: We can take two groups, [-2,-4] and [2,4] to form [-2,-4,2,4] or [2,4,-2,-4].
//
// Constraints:
//
// - 2 <= arr.length <= 3 * 10^4
// - arr.length is even.
// - -10^5 <= arr[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        for &a in &arr {
            *hm.entry(a).or_insert(0) += 1;
        }
        let mut v = hm.keys().cloned().collect::<Vec<_>>();
        v.sort_unstable_by_key(|k| k.abs());
        for k in &v {
            let n1 = *hm.get(k).unwrap();
            if n1 > 0 {
                if let Some(n2) = hm.get_mut(&(k * 2)) {
                    *n2 -= n1;
                    if *n2 < 0 {
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
        (vec![3, 1, 3, 6], false),
        (vec![2, 1, 2, 6], false),
        (vec![4, -2, 2, -4], true),
        (vec![1, 2, 4, 16, 8, 4], false),
    ];
    for (arr, expected) in cases {
        assert_eq!(Solution::can_reorder_doubled(arr), expected);
    }
}
