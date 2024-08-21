#![allow(dead_code)]

// 3261. Count Substrings That Satisfy K-Constraint II
// https://leetcode.com/problems/count-substrings-that-satisfy-k-constraint-ii/
// https://leetcode.cn/problems/count-substrings-that-satisfy-k-constraint-ii/
//
// Hard
//
// You are given a binary string s and an integer k.
//
// You are also given a 2D integer array queries, where queries[i] = [li, ri].
//
// A binary string satisfies the k-constraint if either of the following conditions holds:
//
//     The number of 0's in the string is at most k.
//     The number of 1's in the string is at most k.
//
// Return an integer array answer, where answer[i] is the number of
// substrings of s[li..ri] that satisfy the k-constraint.
//
// Example 1:
//
// Input: s = "0001111", k = 2, queries = [[0,6]]
//
// Output: [26]
//
// Explanation:
//
// For the query [0, 6], all substrings of s[0..6] = "0001111" satisfy the k-constraint
// except for the substrings s[0..5] = "000111" and s[0..6] = "0001111".
//
// Example 2:
//
// Input: s = "010101", k = 1, queries = [[0,5],[1,4],[2,3]]
//
// Output: [15,9,3]
//
// Explanation:
//
// The substrings of s with a length greater than 3 do not satisfy the k-constraint.
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     s[i] is either '0' or '1'.
//     1 <= k <= s.length
//     1 <= queries.length <= 10^5
//     queries[i] == [li, ri]
//     0 <= li <= ri < s.length
//     All queries are distinct.
//

struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let s = s.as_bytes();
        let n = s.len();
        let mut to = vec![0; n];
        let mut j = 0;
        let mut count = (0, 0);
        for i in 0..n {
            while j < n {
                if s[j] == b'0' {
                    if count.0 == k && count.1 > k {
                        break;
                    }
                    count.0 += 1;
                } else {
                    if count.1 == k && count.0 > k {
                        break;
                    }
                    count.1 += 1;
                }
                j += 1;
            }
            to[i] = j;
            if s[i] == b'0' {
                count.0 -= 1;
            } else {
                count.1 -= 1;
            }
        }

        let mut sum = vec![0i64; n];
        sum[0] = to[0] as i64;
        for i in 1..n {
            sum[i] = sum[i - 1] + (to[i] - i) as i64;
        }

        let mut res = Vec::new();
        for q in &queries {
            let (l, r) = (q[0] as usize, q[1] as usize);
            if r < to[0] {
                let len = (r - l + 1) as i64;
                res.push(len * (len + 1) / 2);
            } else {
                let mut mid = to.binary_search(&r).unwrap_or_else(|x| x - 1);
                while to[mid + 1] == r {
                    mid += 1;
                }
                if mid < l {
                    let len = (r - l + 1) as i64;
                    res.push(len * (len + 1) / 2);
                } else {
                    let left = sum[mid] - if l > 0 { sum[l - 1] } else { 0 };
                    let len = (r - mid) as i64;
                    res.push(left + len * (len + 1) / 2);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "0001111".to_string();
    let k = 2;
    let queries = vec![vec![0, 6]];
    let output = vec![26];
    assert_eq!(Solution::count_k_constraint_substrings(s, k, queries), output);

    let s = "010101".to_string();
    let k = 1;
    let queries = vec![vec![0, 5], vec![1, 4], vec![2, 3]];
    let output = vec![15, 9, 3];
    assert_eq!(Solution::count_k_constraint_substrings(s, k, queries), output);
}
