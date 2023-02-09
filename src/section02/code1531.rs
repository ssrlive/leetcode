#![allow(dead_code)]

/*

// 1531. String Compression II
// https://leetcode.com/problems/string-compression-ii/
// https://leetcode.cn/problems/string-compression-ii/
//
// Hard
//
// Run-length encoding is a string compression method that works by replacing consecutive identical characters (repeated 2 or more times) with the concatenation of the character and the number marking the count of the characters (length of the run). For example, to compress the string "aabccc" we replace "aa" by "a2" and replace "ccc" by "c3". Thus the compressed string becomes "a2bc3".

Notice that in this problem, we are not adding '1' after single characters.

Given a string s and an integer k. You need to delete at most k characters from s such that the run-length encoded version of s has minimum length.

Find the minimum length of the run-length encoded version of s after deleting at most k characters.

Example 1:

Input: s = "aaabcccd", k = 2
Output: 4
Explanation: Compressing s without deleting anything will give us "a3bc3d" of length 6. Deleting any of the characters 'a' or 'c' would at most decrease the length of the compressed string to 5, for instance delete 2 'a' then we will have s = "abcccd" which compressed is abc3d. Therefore, the optimal way is to delete 'b' and 'd', then the compressed version of s will be "a3c3" of length 4.

Example 2:

Input: s = "aabbaa", k = 2
Output: 2
Explanation: If we delete both 'b' characters, the resulting compressed string would be "a4" of length 2.

Example 3:

Input: s = "aaaaaaaaaaa", k = 0
Output: 3
Explanation: Since k is zero, we cannot delete anything. The compressed string is "a11" of length 3.

Constraints:

    1 <= s.length <= 100
    0 <= k <= s.length
    s contains only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        use std::collections::HashMap;

        fn dp(
            s: &[u8],
            k: usize,
            i: usize,
            last: u8,
            cnt: usize,
            j: usize,
            memo: &mut HashMap<(usize, u8, usize, usize), i32>,
        ) -> i32 {
            if i == s.len() {
                0
            } else if let Some(additional_len) = memo.get(&(i, last, cnt, j)) {
                *additional_len
            } else {
                let b = s[i];
                let mut rez = 1 + dp(s, k, i + 1, b, 1, j, memo);
                if last == b {
                    let comp = if cnt != 1 && cnt != 9 && cnt != 99 {
                        dp(s, k, i + 1, b, cnt + 1, j, memo)
                    } else {
                        1 + dp(s, k, i + 1, b, cnt + 1, j, memo)
                    };
                    rez = rez.min(comp);
                }
                if j < k {
                    rez = rez.min(dp(s, k, i + 1, last, cnt, j + 1, memo));
                }
                memo.insert((i, last, cnt, j), rez);
                rez
            }
        }

        dp(s.as_bytes(), k as usize, 0, b'A', 0, 0, &mut HashMap::new())
    }
}

#[test]
fn test() {
    let cases = vec![("aaabcccd", 2, 4), ("aabbaa", 2, 2), ("aaaaaaaaaaa", 0, 3)];
    for (s, k, expect) in cases {
        assert_eq!(Solution::get_length_of_optimal_compression(s.to_string(), k), expect);
    }
}
