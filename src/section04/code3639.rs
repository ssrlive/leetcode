#![allow(dead_code)]

// 3639. Minimum Time to Activate String
// https://leetcode.com/problems/minimum-time-to-activate-string/
// https://leetcode.cn/problems/minimum-time-to-activate-string/
//
// Medium
//
// You are given a string s of length n and an integer array order, where order is a permutation of the numbers in the range [0, n - 1].
//
// Starting from time t = 0, replace the character at index order[t] in s with '*' at each time step.
//
// A substring is valid if it contains at least one '*'.
//
// A string is active if the total number of valid substrings is greater than or equal to k.
//
// Return the minimum time t at which the string s becomes active. If it is impossible, return -1.
//
// Example 1:
//
// Input: s = "abc", order = [1,0,2], k = 2
//
// Output: 0
//
// Explanation:
// t	order[t]	Modified s	Valid Substrings	Count	Active
// (Count >= k)
// 0	1	"a*c"	"*", "a*", "*c", "a*c"	4	Yes
//
// The string s becomes active at t = 0. Thus, the answer is 0.
//
// Example 2:
//
// Input: s = "cat", order = [0,2,1], k = 6
//
// Output: 2
//
// Explanation:
// t	order[t]	Modified s	Valid Substrings	Count	Active
// (Count >= k)
// 0	0	"*at"	"*", "*a", "*at"	3	No
// 1	2	"*a*"	"*", "*a", "*a*", "a*", "*"	5	No
// 2	1	"***"	All substrings (contain '*')	6	Yes
//
// The string s becomes active at t = 2. Thus, the answer is 2.
//
// Example 3:
//
// Input: s = "xy", order = [0,1], k = 4
//
// Output: -1
//
// Explanation:
//
// Even after all replacements, it is impossible to obtain k = 4 valid substrings. Thus, the answer is -1.
//
// Constraints:
//
//     1 <= n == s.length <= 10^5
//     order.length == n
//     0 <= order[i] <= n - 1
//     s consists of lowercase English letters.
//     order is a permutation of integers from 0 to n - 1.
//     1 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
        let n = s.len();
        let m = order.len();
        let mut pos: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();
        pos.insert(-1);
        pos.insert(n as i32);

        let mut k = k as i64;
        for (t, &order_t) in order.iter().enumerate().take(m) {
            let i = order_t;

            let mut iter = pos.range(i..);
            let r = *iter.next().unwrap();

            let mut iter_prev = pos.range(..i);
            let l = *iter_prev.next_back().unwrap();

            k -= (i - l) as i64 * (r - i) as i64;
            pos.insert(i);

            if k <= 0 {
                return t as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_time("abc".to_string(), vec![1, 0, 2], 2), 0);
    assert_eq!(Solution::min_time("cat".to_string(), vec![0, 2, 1], 6), 2);
    assert_eq!(Solution::min_time("xy".to_string(), vec![0, 1], 4), -1);
}
