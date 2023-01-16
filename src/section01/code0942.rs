#![allow(dead_code)]

// 942. DI String Match
// https://leetcode.com/problems/di-string-match/
// https://leetcode.cn/problems/di-string-match/
//
// A permutation perm of n + 1 integers of all the integers in the range [0, n] can be represented as a string s of length n where:
//
// s[i] == 'I' if perm[i] < perm[i + 1], and
// s[i] == 'D' if perm[i] > perm[i + 1].
// Given a string s, reconstruct the permutation perm and return it. If there are multiple valid permutations perm, return any of them.
//
// Example 1:
//
// Input: s = "IDID"
// Output: [0,4,1,3,2]
//
// Example 2:
//
// Input: s = "III"
// Output: [0,1,2,3]
//
// Example 3:
//
// Input: s = "DDI"
// Output: [3,2,0,1]
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s[i] is either 'I' or 'D'.
//

struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res = Vec::with_capacity(s.len() + 1);
        let (mut min_val, mut max_val) = (0, s.len() as i32);

        s.as_bytes().iter().for_each(|b| match *b {
            b'I' => {
                res.push(min_val);
                min_val += 1
            }
            _ => {
                res.push(max_val);
                max_val -= 1
            }
        });
        res.push(max_val);
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::di_string_match("IDID".to_string()), vec![0, 4, 1, 3, 2]);
    assert_eq!(Solution::di_string_match("III".to_string()), vec![0, 1, 2, 3]);
    assert_eq!(Solution::di_string_match("DDI".to_string()), vec![3, 2, 0, 1]);
}
