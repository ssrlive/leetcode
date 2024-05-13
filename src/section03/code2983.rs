#![allow(dead_code)]

// 2983. Palindrome Rearrangement Queries
// https://leetcode.com/problems/palindrome-rearrangement-queries/
// https://leetcode.cn/problems/palindrome-rearrangement-queries/
//
// Hard
//
// You are given a 0-indexed string s having an even length n.
//
// You are also given a 0-indexed 2D integer array, queries, where queries[i] = [ai, bi, ci, di].
//
// For each query i, you are allowed to perform the following operations:
//
// Rearrange the characters within the substring s[ai:bi], where 0 <= ai <= bi < n / 2.
// Rearrange the characters within the substring s[ci:di], where n / 2 <= ci <= di < n.
// For each query, your task is to determine whether it is possible to make s a palindrome by performing the operations.
//
// Each query is answered independently of the others.
//
// Return a 0-indexed array answer, where answer[i] == true if it is possible to make s a palindrome
// by performing operations specified by the ith query, and false otherwise.
//
// A substring is a contiguous sequence of characters within a string.
// s[x:y] represents the substring consisting of characters from the index x to index y in s, both inclusive.
//
//
// Example 1:
//
// Input: s = "abcabc", queries = [[1,1,3,5],[0,2,5,5]]
// Output: [true,true]
// Explanation: In this example, there are two queries:
// In the first query:
// - a0 = 1, b0 = 1, c0 = 3, d0 = 5.
// - So, you are allowed to rearrange s[1:1] => abcabc and s[3:5] => abcabc.
// - To make s a palindrome, s[3:5] can be rearranged to become => abccba.
// - Now, s is a palindrome. So, answer[0] = true.
// In the second query:
// - a1 = 0, b1 = 2, c1 = 5, d1 = 5.
// - So, you are allowed to rearrange s[0:2] => abcabc and s[5:5] => abcabc.
// - To make s a palindrome, s[0:2] can be rearranged to become => cbaabc.
// - Now, s is a palindrome. So, answer[1] = true.
//
// Example 2:
//
// Input: s = "abbcdecbba", queries = [[0,2,7,9]]
// Output: [false]
// Explanation: In this example, there is only one query.
// a0 = 0, b0 = 2, c0 = 7, d0 = 9.
// So, you are allowed to rearrange s[0:2] => abbcdecbba and s[7:9] => abbcdecbba.
// It is not possible to make s a palindrome by rearranging these substrings because s[3:6] is not a palindrome.
// So, answer[0] = false.
//
// Example 3:
//
// Input: s = "acbcab", queries = [[1,2,4,5]]
// Output: [true]
// Explanation: In this example, there is only one query.
// a0 = 1, b0 = 2, c0 = 4, d0 = 5.
// So, you are allowed to rearrange s[1:2] => acbcab and s[4:5] => acbcab.
// To make s a palindrome s[1:2] can be rearranged to become abccab.
// Then, s[4:5] can be rearranged to become abccba.
// Now, s is a palindrome. So, answer[0] = true.
//
// Constraints:
//
// 2 <= n == s.length <= 10^5
// 1 <= queries.length <= 10^5
// queries[i].length == 4
// ai == queries[i][0], bi == queries[i][1]
// ci == queries[i][2], di == queries[i][3]
// 0 <= ai <= bi < n / 2
// n / 2 <= ci <= di < n
// n is even.
// s consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = s.len();
        let q = queries.len();

        let s = s.as_bytes();

        let mut nokv = vec![0; n];
        for i in 0..n {
            nokv[i] = if i == 0 { 0 } else { nokv[i - 1] } + if s[i] != s[n - i - 1] { 1 } else { 0 };
        }

        let okay = |i: i32, j: i32| -> bool {
            if i > j {
                return true;
            }
            nokv[j as usize] - if i == 0 { 0 } else { nokv[(i - 1) as usize] } == 0
        };

        let mut cnt = vec![vec![0; 26]; n];
        for i in 0..n {
            #[allow(clippy::assigning_clones)]
            if i > 0 {
                cnt[i] = cnt[i - 1].clone();
            }
            cnt[i][(s[i] - b'a') as usize] += 1;
        }

        let chkc = |a: i32, b: i32, c: i32, d: i32| -> bool {
            for j in 0..26 {
                let v1 = cnt[b as usize][j as usize] - if a == 0 { 0 } else { cnt[(a - 1) as usize][j as usize] };
                let v2 = cnt[d as usize][j as usize] - if c == 0 { 0 } else { cnt[(c - 1) as usize][j as usize] };
                if v1 != v2 {
                    return false;
                }
            }
            true
        };

        let le = |v1: &Vec<i32>, v2: &Vec<i32>| -> bool {
            for j in 0..v1.len() {
                if v1[j] > v2[j] {
                    return false;
                }
            }
            true
        };

        let sub = |v1: &Vec<i32>, v2: &Vec<i32>| -> Vec<i32> {
            let mut res = v1.clone();
            for j in 0..v1.len() {
                res[j] -= v2[j];
            }
            res
        };

        let gv = |i: i32, j: i32| -> Vec<i32> {
            if i > j {
                return vec![0; 26];
            }
            let mut res = cnt[j as usize].clone();
            if i > 0 {
                res = sub(&res, &cnt[(i - 1) as usize]);
            }
            res
        };

        let spacp = |a: i32, b: i32, c: i32, d: i32| -> bool {
            let n = n as i32;
            let _ap = n - b - 1;
            let bp = n - a - 1;
            let _cp = n - d - 1;
            let dp = n - c - 1;

            let aa = gv(b + 1, dp);
            let bb = gv(c, d);
            if !le(&aa, &bb) {
                return false;
            }
            let cc = sub(&bb, &aa);

            let dd = gv(d + 1, bp);
            let ee = gv(a, b);
            if !le(&dd, &ee) {
                return false;
            }
            let ff = sub(&ee, &dd);

            cc == ff
        };

        let spcpa = |a: i32, b: i32, c: i32, d: i32| -> bool {
            let n = n as i32;
            let ap = n - b - 1;
            let _bp = n - a - 1;
            let cp = n - d - 1;
            let _dp = n - c - 1;

            let aa = gv(cp, a - 1);
            let bb = gv(c, d);
            if !le(&aa, &bb) {
                return false;
            }
            let cc = sub(&bb, &aa);

            let dd = gv(ap, c - 1);
            let ee = gv(a, b);
            if !le(&dd, &ee) {
                return false;
            }
            let ff = sub(&ee, &dd);

            cc == ff
        };

        let ga = |a: i32, b: i32, c: i32, d: i32| -> bool {
            let n = n as i32;
            let ap = n - b - 1;
            let bp = n - a - 1;
            let cp = n - d - 1;
            let dp = n - c - 1;
            if a <= cp && dp <= b {
                let mut ok = true;
                ok = ok && okay(0, a - 1);
                ok = ok && okay(b + 1, n / 2 - 1);
                ok = ok && chkc(a, b, ap, bp);
                return ok;
            }
            if cp <= a && b <= dp {
                let mut ok = true;
                ok = ok && okay(0, cp - 1);
                ok = ok && okay(dp + 1, n / 2 - 1);
                ok = ok && chkc(cp, dp, c, d);
                return ok;
            }

            if b < cp {
                let mut ok = true;
                ok = ok && okay(0, a - 1);
                ok = ok && okay(b + 1, cp - 1);
                ok = ok && okay(dp + 1, n / 2 - 1);
                ok = ok && chkc(a, b, ap, bp);
                ok = ok && chkc(cp, dp, c, d);
                return ok;
            }
            if dp < a {
                let mut ok = true;
                ok = ok && okay(0, cp - 1);
                ok = ok && okay(dp + 1, a - 1);
                ok = ok && okay(b + 1, n / 2 - 1);
                ok = ok && chkc(a, b, ap, bp);
                ok = ok && chkc(cp, dp, c, d);
                return ok;
            }

            if a < cp {
                let mut ok = true;
                ok = ok && okay(0, a - 1);
                ok = ok && okay(dp + 1, n / 2 - 1);
                ok = ok && spacp(a, b, c, d);
                return ok;
            }
            if cp < a {
                let mut ok = true;
                ok = ok && okay(0, cp - 1);
                ok = ok && okay(b + 1, n / 2 - 1);
                ok = ok && spcpa(a, b, c, d);
                return ok;
            }
            false
        };

        let mut res = vec![false; q];
        for i in 0..q {
            let a = queries[i][0];
            let b = queries[i][1];
            let c = queries[i][2];
            let d = queries[i][3];
            res[i] = ga(a, b, c, d);
        }

        res
    }
}

#[test]
fn test() {
    let s = "abcabc".to_string();
    let queries = vec![vec![1, 1, 3, 5], vec![0, 2, 5, 5]];
    let res = vec![true, true];
    assert_eq!(Solution::can_make_palindrome_queries(s, queries), res);

    let s = "abbcdecbba".to_string();
    let queries = vec![vec![0, 2, 7, 9]];
    let res = vec![false];
    assert_eq!(Solution::can_make_palindrome_queries(s, queries), res);

    let s = "acbcab".to_string();
    let queries = vec![vec![1, 2, 4, 5]];
    let res = vec![true];
    assert_eq!(Solution::can_make_palindrome_queries(s, queries), res);
}
