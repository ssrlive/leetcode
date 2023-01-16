#![allow(dead_code)]

// 1189. Maximum Number of Balloons
// https://leetcode.com/problems/maximum-number-of-balloons/
// https://leetcode.cn/problems/maximum-number-of-balloons/
//
// Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.
//
// You can use each character in text at most once. Return the maximum number of instances that can be formed.
//
// Example 1:
//
// Input: text = "nlaebolko"
// Output: 1
//
// Example 2:
//
// Input: text = "loonbalxballpoon"
// Output: 2
//
// Example 3:
//
// Input: text = "leetcode"
// Output: 0
//
// Constraints:
//
// - 1 <= text.length <= 10^4
// - text consists of lower case English letters only.
//

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        use std::collections::HashMap;

        let mut counter: HashMap<char, i32> = "balloon".chars().map(|c| (c, 0)).collect();

        text.chars().for_each(|c| {
            counter.entry(c).and_modify(|e| *e += 1);
        });

        counter
            .iter()
            .map(|(&c, &num)| num / if "lo".contains(c) { 2 } else { 1 })
            .min()
            .unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
    assert_eq!(Solution::max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
}
