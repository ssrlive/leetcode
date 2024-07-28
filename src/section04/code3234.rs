#![allow(dead_code)]

// 3234. Count the Number of Substrings With Dominant Ones
// https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/
// https://leetcode.cn/problems/count-the-number-of-substrings-with-dominant-ones/
//
// Medium
//
// You are given a binary string s.
//
// Return the number of substrings with dominant ones.
//
// A string has dominant ones if the number of ones in the string is greater
// than or equal to the square of the number of zeros in the string.
//
// Example 1:
//
// Input: s = "00011"
//
// Output: 5
//
// Explanation:
//
// The substrings with dominant ones are shown in the table below.
// i	j	s[i..j]	Number of Zeros	Number of Ones
// 3	3	1	0	1
// 4	4	1	0	1
// 2	3	01	1	1
// 3	4	11	0	2
// 2	4	011	1	2
//
// Example 2:
//
// Input: s = "101101"
//
// Output: 16
//
// Explanation:
//
// The substrings with non-dominant ones are shown in the table below.
//
// Since there are 21 substrings total and 5 of them have non-dominant ones,
// it follows that there are 16 substrings with dominant ones.
// i	j	s[i..j]	Number of Zeros	Number of Ones
// 1	1	0	1	0
// 4	4	0	1	0
// 1	4	0110	2	2
// 0	4	10110	2	3
// 1	5	01101	2	3
//
// Constraints:
//
//     1 <= s.length <= 4 * 10^4
//     s consists only of characters '0' and '1'.
//

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;
        let mut z = 0;
        while z + z * z <= s.len() {
            let mut cnt = [0; 2];
            let mut p = 0;
            let mut i = 0;
            let mut j = 0;
            while i < s.len() {
                cnt[(s[i] == b'1') as usize] += 1;
                while cnt[0] > z {
                    cnt[(s[j] == b'1') as usize] -= 1;
                    j += 1;
                }
                if cnt[0] == z && cnt[1] > 0 && cnt[1] >= z * z {
                    p = p.max(j);
                    while p < i && s[p] == b'1' {
                        p += 1;
                    }
                    res += 1 + (p - j).min(cnt[1] - z * z);
                }
                i += 1;
            }
            z += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_substrings("00011".to_string()), 5);
    assert_eq!(Solution::number_of_substrings("101101".to_string()), 16);
}
