#![allow(dead_code)]

/*

// 1864. Minimum Number of Swaps to Make the Binary String Alternating
// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-binary-string-alternating/
// https://leetcode.cn/problems/minimum-number-of-swaps-to-make-the-binary-string-alternating/
//
// Medium
//
// Given a binary string s, return the minimum number of character swaps to make it alternating, or -1 if it is impossible.

The string is called alternating if no two adjacent characters are equal. For example, the strings "010" and "1010" are alternating, while the string "0100" is not.

Any two characters may be swapped, even if they are not adjacent.

Example 1:

Input: s = "111000"
Output: 1
Explanation: Swap positions 1 and 4: "111000" -> "101010"
The string is now alternating.

Example 2:

Input: s = "010"
Output: 0
Explanation: The string is already alternating, no swaps are needed.

Example 3:

Input: s = "1110"
Output: -1

Constraints:

    1 <= s.length <= 1000
    s[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut count_0 = 0_i32;
        let mut count_1 = 0;
        for &s_i in s.iter() {
            if s_i == '1' {
                count_1 += 1;
            } else {
                count_0 += 1;
            }
        }
        if (count_1 - count_0).abs() >= 2 {
            return -1;
        }
        let mut uncommon_1 = 0;
        for (i, &s_i) in s.iter().enumerate() {
            if i.is_multiple_of(2) {
                if s_i != '0' {
                    uncommon_1 += 1;
                }
            } else if s_i != '1' {
                uncommon_1 += 1;
            }
        }
        let mut uncommon_2 = 0;
        for (i, &s_i) in s.iter().enumerate() {
            if i % 2 == 1 {
                if s_i != '0' {
                    uncommon_2 += 1;
                }
            } else if s_i != '1' {
                uncommon_2 += 1;
            }
        }
        let mut mini = i32::MAX;
        if uncommon_1 % 2 == 0 {
            mini = mini.min(uncommon_1);
        }
        if uncommon_2 % 2 == 0 {
            mini = mini.min(uncommon_2);
        }
        if mini == i32::MAX {
            return -1;
        }
        mini / 2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_swaps("111000".to_string()), 1);
    assert_eq!(Solution::min_swaps("010".to_string()), 0);
    assert_eq!(Solution::min_swaps("1110".to_string()), -1);
}
