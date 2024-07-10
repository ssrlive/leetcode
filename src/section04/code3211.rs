#![allow(dead_code)]

// 3211. Generate Binary Strings Without Adjacent Zeros
// https://leetcode.com/problems/generate-binary-strings-without-adjacent-zeros/
// https://leetcode.cn/problems/generate-binary-strings-without-adjacent-zeros/
//
// Medium
//
// You are given a positive integer n.
//
// A binary string x is valid if all substrings of x of length 2 contain at least one "1".
//
// Return all valid strings with length n, in any order.
//
// Example 1:
//
// Input: n = 3
//
// Output: ["010","011","101","110","111"]
//
// Explanation:
//
// The valid strings of length 3 are: "010", "011", "101", "110", and "111".
//
// Example 2:
//
// Input: n = 1
//
// Output: ["0","1"]
//
// Explanation:
//
// The valid strings of length 1 are: "0" and "1".
//
// Constraints:
//
//     1 <= n <= 18
//

struct Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        if n == 1 {
            vec!["0".to_string(), "1".to_string()]
        } else {
            Self::valid_strings(n - 1).into_iter().fold(vec![], |mut v, mut s| {
                if s.ends_with('1') {
                    let mut x = s.clone();
                    v.push({
                        x.push('0');
                        x
                    });
                }
                v.push({
                    s.push('1');
                    s
                });
                v
            })
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::valid_strings(3),
        vec![
            "010".to_string(),
            "011".to_string(),
            "101".to_string(),
            "110".to_string(),
            "111".to_string()
        ]
    );
    assert_eq!(Solution::valid_strings(1), vec!["0".to_string(), "1".to_string()]);
}
