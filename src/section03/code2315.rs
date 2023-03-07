#![allow(dead_code)]

/*

// 2315. Count Asterisks
// https://leetcode.com/problems/count-asterisks/
// https://leetcode.cn/problems/count-asterisks/
//
// Easy
//
// You are given a string s, where every two consecutive vertical bars '|' are grouped into a pair.
// In other words, the 1st and 2nd '|' make a pair, the 3rd and 4th '|' make a pair, and so forth.

Return the number of '*' in s, excluding the '*' between each pair of '|'.

Note that each '|' will belong to exactly one pair.

Example 1:

Input: s = "l|*e*et|c**o|*de|"
Output: 2
Explanation: The considered characters are underlined: "l|*e*et|c**o|*de|".
The characters between the first and second '|' are excluded from the answer.
Also, the characters between the third and fourth '|' are excluded from the answer.
There are 2 asterisks considered. Therefore, we return 2.

Example 2:

Input: s = "iamprogrammer"
Output: 0
Explanation: In this example, there are no asterisks in s. Therefore, we return 0.

Example 3:

Input: s = "yo|uar|e**|b|e***au|tifu|l"
Output: 5
Explanation: The considered characters are underlined: "yo|uar|e**|b|e***au|tifu|l".
             There are 5 asterisks considered. Therefore, we return 5.

Constraints:

    1 <= s.length <= 1000
    s consists of lowercase English letters, vertical bars '|', and asterisks '*'.
    s contains an even number of vertical bars '|'.
*/

struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .flat_map(|(_, s)| s.chars())
            .filter(|&c| c == '*')
            .count() as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()), 2);
    assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
    assert_eq!(Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()), 5);
}
