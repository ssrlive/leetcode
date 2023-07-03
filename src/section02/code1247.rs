#![allow(dead_code)]

// 1247. Minimum Swaps to Make Strings Equal
// https://leetcode.com/problems/minimum-swaps-to-make-strings-equal/
// https://leetcode.cn/problems/minimum-swaps-to-make-strings-equal/
//
// Medium
//
// You are given two strings s1 and s2 of equal length consisting of letters "x" and "y" only. Your task is to make these two strings equal to each other. You can swap any two characters that belong to different strings, which means: swap s1[i] and s2[j].
//
// Return the minimum number of swaps required to make s1 and s2 equal, or return -1 if it is impossible to do so.
//
// Example 1:
//
// Input: s1 = "xx", s2 = "yy"
// Output: 1
// Explanation: Swap s1[0] and s2[1], s1 = "yx", s2 = "yx".
//
// Example 2:
//
// Input: s1 = "xy", s2 = "yx"
// Output: 2
// Explanation: Swap s1[0] and s2[0], s1 = "yy", s2 = "xx".
// Swap s1[0] and s2[1], s1 = "xy", s2 = "xy".
// Note that you cannot swap s1[0] and s1[1] to make s1 equal to "yx", cause we can only swap chars in different strings.
//
// Example 3:
//
// Input: s1 = "xx", s2 = "xy"
// Output: -1
//
// Constraints:
//
// -    1 <= s1.length, s2.length <= 1000
// -    s1, s2 only contain 'x' or 'y'.
//

struct Solution;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut x1 = 0;
        let mut y1 = 0;
        // let mut _x2 = 0;
        // let mut _y2 = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == 'x' && c2 == 'y' {
                x1 += 1;
            } else if c1 == 'y' && c2 == 'x' {
                y1 += 1;
                // } else if c1 == 'x' && c2 == 'x' {
                //     _x2 += 1;
                // } else if c1 == 'y' && c2 == 'y' {
                //     _y2 += 1;
            }
        }
        let mut ans = 0;
        if x1 % 2 == 1 && y1 % 2 == 1 {
            ans += 2;
            x1 -= 1;
            y1 -= 1;
        }
        ans += x1 / 2 + y1 / 2;
        if x1 % 2 == 1 || y1 % 2 == 1 {
            return -1;
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![("xx", "yy", 1), ("xy", "yx", 2), ("xx", "xy", -1), ("xxyyxyxyxx", "xyyxyxxxyx", 4)];
    for (s1, s2, expected) in cases {
        let s1 = s1.to_string();
        let s2 = s2.to_string();
        assert_eq!(Solution::minimum_swap(s1, s2), expected);
    }
}
