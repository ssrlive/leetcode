#![allow(dead_code)]

// 3579. Minimum Steps to Convert String with Operations
// https://leetcode.com/problems/minimum-steps-to-convert-string-with-operations/
// https://leetcode.cn/problems/minimum-steps-to-convert-string-with-operations/
//
// Hard
//
// You are given two strings, word1 and word2, of equal length. You need to transform word1 into word2.
//
// For this, divide word1 into one or more contiguous
//
// . For each substring substr you can perform the following operations:
//
//     Replace: Replace the character at any one index of substr with another lowercase English letter.
//
//     Swap: Swap any two characters in substr.
//
//     Reverse Substring: Reverse substr.
//
// Each of these counts as one operation and each character of each substring can be used in each type of operation at most once (i.e. no single index may be involved in more than one replace, one swap, or one reverse).
//
// Return the minimum number of operations required to transform word1 into word2.
//
// Example 1:
//
// Input: word1 = "abcdf", word2 = "dacbe"
//
// Output: 4
//
// Explanation:
//
// Divide word1 into "ab", "c", and "df". The operations are:
//
//     For the substring "ab",
//         Perform operation of type 3 on "ab" -> "ba".
//         Perform operation of type 1 on "ba" -> "da".
//     For the substring "c" do no operations.
//     For the substring "df",
//         Perform operation of type 1 on "df" -> "bf".
//         Perform operation of type 1 on "bf" -> "be".
//
// Example 2:
//
// Input: word1 = "abceded", word2 = "baecfef"
//
// Output: 4
//
// Explanation:
//
// Divide word1 into "ab", "ce", and "ded". The operations are:
//
//     For the substring "ab",
//         Perform operation of type 2 on "ab" -> "ba".
//     For the substring "ce",
//         Perform operation of type 2 on "ce" -> "ec".
//     For the substring "ded",
//         Perform operation of type 1 on "ded" -> "fed".
//         Perform operation of type 1 on "fed" -> "fef".
//
// Example 3:
//
// Input: word1 = "abcdef", word2 = "fedabc"
//
// Output: 2
//
// Explanation:
//
// Divide word1 into "abcdef". The operations are:
//
//     For the substring "abcdef",
//         Perform operation of type 3 on "abcdef" -> "fedcba".
//         Perform operation of type 2 on "fedcba" -> "fedabc".
//
// Constraints:
//
//     1 <= word1.length == word2.length <= 100
//     word1 and word2 consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let mut ops = vec![vec![vec![0; 2]; n]; n];

        for (l, ops_l) in ops.iter_mut().enumerate().take(n) {
            for (r, ops_l_r) in ops_l.iter_mut().enumerate().take(n).skip(l) {
                for rev in [false, true] {
                    let mut save = [[0; 26]; 26];
                    for idx in l..=r {
                        let w1 = if rev { r - (idx - l) } else { idx };
                        if word1.as_bytes()[w1] == word2.as_bytes()[idx] {
                            continue;
                        }
                        let c1 = (word1.as_bytes()[w1] - b'a') as usize;
                        let c2 = (word2.as_bytes()[idx] - b'a') as usize;
                        if save[c2][c1] > 0 {
                            save[c2][c1] -= 1;
                        } else {
                            save[c1][c2] += 1;
                            ops_l_r[rev as usize] += 1;
                        }
                    }
                }
            }
        }

        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            dp[i] = i32::MAX;
            for r in i..n {
                dp[i] = dp[i].min(dp[r + 1] + ops[i][r][0].min(1 + ops[i][r][1]));
            }
        }
        dp[0]
    }
}

#[test]
fn test() {
    let word1 = "abcdf".to_string();
    let word2 = "dacbe".to_string();
    assert_eq!(Solution::min_operations(word1, word2), 4);

    let word1 = "abceded".to_string();
    let word2 = "baecfef".to_string();
    assert_eq!(Solution::min_operations(word1, word2), 4);

    let word1 = "abcdef".to_string();
    let word2 = "fedabc".to_string();
    assert_eq!(Solution::min_operations(word1, word2), 2);
}
