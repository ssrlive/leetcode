#![allow(dead_code)]

// 2977. Minimum Cost to Convert String II
// https://leetcode.com/problems/minimum-cost-to-convert-string-ii/
// https://leetcode.cn/problems/minimum-cost-to-convert-string-ii/
//
// Hard
//
// You are given two 0-indexed strings source and target, both of length n and consisting of lowercase English characters.
// You are also given two 0-indexed string arrays original and changed, and an integer array cost,
// where cost[i] represents the cost of converting the string original[i] to the string changed[i].
//
// You start with the string source. In one operation, you can pick a substring x from the string,
// and change it to y at a cost of z if there exists any index j such that cost[j] == z, original[j] == x, and changed[j] == y.
// You are allowed to do any number of operations, but any pair of operations must satisfy either of these two conditions:
//
//    - The substrings picked in the operations are source[a..b] and source[c..d] with either b < c or d < a.
//      In other words, the indices picked in both operations are disjoint.
//    - The substrings picked in the operations are source[a..b] and source[c..d] with a == c and b == d. In other words,
//      the indices picked in both operations are identical.
//
// Return the minimum cost to convert the string source to the string target using any number of operations.
// If it is impossible to convert source to target, return -1.
//
// Note that there may exist indices i, j such that original[j] == original[i] and changed[j] == changed[i].
//
// Example 1:
//
// Input: source = "abcd", target = "acbe", original = ["a","b","c","c","e","d"], changed = ["b","c","b","e","b","e"], cost = [2,5,5,1,2,20]
// Output: 28
// Explanation: To convert "abcd" to "acbe", do the following operations:
// - Change substring source[1..1] from "b" to "c" at a cost of 5.
// - Change substring source[2..2] from "c" to "e" at a cost of 1.
// - Change substring source[2..2] from "e" to "b" at a cost of 2.
// - Change substring source[3..3] from "d" to "e" at a cost of 20.
// The total cost incurred is 5 + 1 + 2 + 20 = 28.
// It can be shown that this is the minimum possible cost.
//
// Example 2:
//
// Input: source = "abcdefgh", target = "acdeeghh", original = ["bcd","fgh","thh"], changed = ["cde","thh","ghh"], cost = [1,3,5]
// Output: 9
// Explanation: To convert "abcdefgh" to "acdeeghh", do the following operations:
// - Change substring source[1..3] from "bcd" to "cde" at a cost of 1.
// - Change substring source[5..7] from "fgh" to "thh" at a cost of 3.
//   We can do this operation because indices [5,7] are disjoint with indices picked in the first operation.
// - Change substring source[5..7] from "thh" to "ghh" at a cost of 5.
//   We can do this operation because indices [5,7] are disjoint with indices picked in the first operation,
//   and identical with indices picked in the second operation.
// The total cost incurred is 1 + 3 + 5 = 9.
// It can be shown that this is the minimum possible cost.
//
// Example 3:
//
// Input: source = "abcdefgh", target = "addddddd", original = ["bcd","defgh"], changed = ["ddd","ddddd"], cost = [100,1578]
// Output: -1
// Explanation: It is impossible to convert "abcdefgh" to "addddddd".
// If you select substring source[1..3] as the first operation to change "abcdefgh" to "adddefgh",
// you cannot select substring source[3..7] as the second operation because it has a common index, 3, with the first operation.
// If you select substring source[3..7] as the first operation to change "abcdefgh" to "abcddddd",
// you cannot select substring source[1..3] as the second operation because it has a common index, 3, with the first operation.
//
// Constraints:
//
//     1 <= source.length == target.length <= 1000
//     source, target consist only of lowercase English characters.
//     1 <= cost.length == original.length == changed.length <= 100
//     1 <= original[i].length == changed[i].length <= source.length
//     original[i], changed[i] consist only of lowercase English characters.
//     original[i] != changed[i]
//     1 <= cost[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<String>, changed: Vec<String>, cost: Vec<i32>) -> i64 {
        use std::cmp::min;
        use std::collections::{HashMap, HashSet};

        let mut l = HashSet::new();
        let mut h = HashSet::new();

        l.insert(1);

        for i in 0..original.len() {
            l.insert(original[i].len());

            h.insert(original[i].as_str());
            h.insert(changed[i].as_str());
        }

        let mut l = l.into_iter().collect::<Vec<_>>();

        l.sort();

        let n = h.len();
        let h = h.into_iter().enumerate().fold(HashMap::new(), |mut h, t| {
            h.insert(t.1, t.0);

            h
        });

        let mut d = vec![vec![i32::MAX; n]; n];

        for (i, d_i) in d.iter_mut().enumerate() {
            d_i[i] = 0;
        }

        for k in 0..original.len() {
            let i = h[original[k].as_str()];
            let j = h[changed[k].as_str()];

            d[i][j] = min(d[i][j], cost[k]);
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if d[i][k] < i32::MAX && d[k][j] < i32::MAX {
                        d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
                    }
                }
            }
        }

        let n = source.len();
        let mut dp = vec![-1; n + 1];

        dp[0] = 0;

        for i in 0..n {
            if dp[i] >= 0 {
                for &l_k in &l {
                    let j = i + l_k;

                    if j <= n {
                        let ss = &source[i..j];
                        let st = &target[i..j];

                        let mut c = 0;

                        if ss != st {
                            if h.contains_key(ss) && h.contains_key(st) {
                                let si = h[ss];
                                let ti = h[st];

                                c = d[si][ti];

                                if c == i32::MAX {
                                    c = -1;
                                }
                            } else {
                                c = -1;
                            }
                        }

                        if c >= 0 && (dp[j] == -1 || dp[j] > dp[i] + c as i64) {
                            dp[j] = dp[i] + c as i64;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        dp[n]
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::minimum_cost(
            "abcd".to_string(),
            "acbe".to_string(),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "c".to_string(),
                "e".to_string(),
                "d".to_string()
            ],
            vec![
                "b".to_string(),
                "c".to_string(),
                "b".to_string(),
                "e".to_string(),
                "b".to_string(),
                "e".to_string()
            ],
            vec![2, 5, 5, 1, 2, 20]
        ),
        28
    );

    assert_eq!(
        Solution::minimum_cost(
            "abcdefgh".to_string(),
            "acdeeghh".to_string(),
            vec!["bcd".to_string(), "fgh".to_string(), "thh".to_string()],
            vec!["cde".to_string(), "thh".to_string(), "ghh".to_string()],
            vec![1, 3, 5]
        ),
        9
    );

    assert_eq!(
        Solution::minimum_cost(
            "abcdefgh".to_string(),
            "addddddd".to_string(),
            vec!["bcd".to_string(), "defgh".to_string()],
            vec!["ddd".to_string(), "ddddd".to_string()],
            vec![100, 1578]
        ),
        -1
    );
}
