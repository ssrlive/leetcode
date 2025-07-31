#![allow(dead_code)]

// 3628. Maximum Number of Subsequences After One Inserting
// https://leetcode.com/problems/maximum-number-of-subsequences-after-one-inserting/
// https://leetcode.cn/problems/maximum-number-of-subsequences-after-one-inserting/
//
// Medium
//
// You are given a string s consisting of uppercase English letters.
//
// You are allowed to insert at most one uppercase English letter at any position (including the beginning or end) of the string.
//
// Return the maximum number of "LCT" subsequences that can be formed in the resulting string after at most one insertion.
//
// Example 1:
//
// Input: s = "LMCT"
//
// Output: 2
//
// Explanation:
//
// We can insert a "L" at the beginning of the string s to make "LLMCT", which has 2 subsequences, at indices [0, 3, 4] and [1, 3, 4].
//
// Example 2:
//
// Input: s = "LCCT"
//
// Output: 4
//
// Explanation:
//
// We can insert a "L" at the beginning of the string s to make "LLCCT", which has 4 subsequences, at indices [0, 2, 4], [0, 3, 4], [1, 2, 4] and [1, 3, 4].
//
// Example 3:
//
// Input: s = "L"
//
// Output: 0
//
// Explanation:
//
// Since it is not possible to obtain the subsequence "LCT" by inserting a single letter, the result is 0.
//
// Constraints:
//
// 1 <= s.length <= 10^5
// s consists of uppercase English letters.
//

struct Solution;

impl Solution {
    pub fn num_of_subsequences(s: String) -> i64 {
        fn max_extra_when_add_c(s: &[u8], init_t: i64) -> i64 {
            let (mut cnt_l, mut cnt_t, mut ans) = (0, init_t, 0);
            for &c in s {
                match c {
                    b'L' => cnt_l += 1,
                    b'T' => cnt_t -= 1,
                    _ => (),
                }
                ans = ans.max(cnt_t * cnt_l);
            }
            ans
        }

        fn num(s: &[u8], init_l: i64, init_t: i64) -> i64 {
            let (mut cnt_l, mut cnt_t, mut ans) = (init_l, init_t, 0);
            for &c in s {
                match c {
                    b'L' => cnt_l += 1,
                    b'C' => ans += cnt_l * cnt_t,
                    b'T' => cnt_t -= 1,
                    _ => (),
                }
            }
            ans
        }

        let s = s.as_bytes();
        let cnt_t = s.iter().filter(|&&c| c == b'T').count() as i64;
        let result_when_add_l = num(s, 1, cnt_t);
        let result_when_add_c = num(s, 0, cnt_t) + max_extra_when_add_c(s, cnt_t);
        let result_when_add_t = num(s, 0, cnt_t + 1);
        result_when_add_t.max(result_when_add_l).max(result_when_add_c)
    }
}

#[test]
fn test() {
    let s = "LMCT".to_string();
    assert_eq!(Solution::num_of_subsequences(s), 2);

    let s = "LCCT".to_string();
    assert_eq!(Solution::num_of_subsequences(s), 4);

    let s = "L".to_string();
    assert_eq!(Solution::num_of_subsequences(s), 0);
}
