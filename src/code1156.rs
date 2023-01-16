#![allow(dead_code)]

// 1156. Swap For Longest Repeated Character Substring
// https://leetcode.com/problems/swap-for-longest-repeated-character-substring/
// https://leetcode.cn/problems/swap-for-longest-repeated-character-substring/
//
// You are given a string text. You can swap two of the characters in the text.
//
// Return the length of the longest substring with repeated characters.
//
// Example 1:
//
// Input: text = "ababa"
// Output: 3
// Explanation: We can swap the first 'b' with the last 'a', or the last 'b' with the first 'a'. Then, the longest repeated character substring is "aaa" with length 3.
//
// Example 2:
//
// Input: text = "aaabaaa"
// Output: 6
// Explanation: Swap 'b' with the last 'a' (or the first 'a'), and we get longest repeated character substring "aaaaaa" with length 6.
//
// Example 3:
//
// Input: text = "aaaaa"
// Output: 5
// Explanation: No need to swap, longest repeated character substring is "aaaaa" with length is 5.
//
// Constraints:
//
// - 1 <= text.length <= 2 * 10^4
// - text consist of lowercase English characters only.
//

struct Solution;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        use std::collections::HashMap;

        fn good(w: &HashMap<char, usize>, m: &HashMap<char, usize>) -> bool {
            if w.len() == 1 {
                return true;
            }
            if w.len() > 2 {
                return false;
            }
            let mut a = '#';
            let mut b = '#';
            for k in w.keys() {
                if a == '#' {
                    a = *k;
                } else {
                    b = *k;
                }
            }
            if w[&a] == 1 && m.contains_key(&b) || w[&b] == 1 && m.contains_key(&a) {
                return true;
            }
            false
        }

        let mut m = HashMap::<char, usize>::new();
        for c in text.chars() {
            *m.entry(c).or_default() += 1;
        }

        let mut i = 0;
        let mut ans = 0;
        let mut w = HashMap::<char, usize>::new();
        for (j, c) in text.chars().enumerate() {
            *w.entry(c).or_default() += 1;
            *m.entry(c).or_default() -= 1;
            if m[&c] == 0 {
                m.remove(&c);
            }

            while !good(&w, &m) {
                *w.entry(text.chars().nth(i).unwrap()).or_default() -= 1;
                *m.entry(text.chars().nth(i).unwrap()).or_default() += 1;
                if w[&text.chars().nth(i).unwrap()] == 0 {
                    w.remove(&text.chars().nth(i).unwrap());
                }
                i += 1;
            }
            ans = ans.max(j - i + 1);
        }
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_rep_opt1("ababa".to_string()), 3);
    assert_eq!(Solution::max_rep_opt1("aaabaaa".to_string()), 6);
    assert_eq!(Solution::max_rep_opt1("aaaaa".to_string()), 5);
}
