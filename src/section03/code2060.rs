#![allow(dead_code)]

/*

// 2060. Check if an Original String Exists Given Two Encoded Strings
// https://leetcode.com/problems/check-if-an-original-string-exists-given-two-encoded-strings/
// https://leetcode.cn/problems/check-if-an-original-string-exists-given-two-encoded-strings/
//
// Hard
//
// An original string, consisting of lowercase English letters, can be encoded by the following steps:

    Arbitrarily split it into a sequence of some number of non-empty substrings.
    Arbitrarily choose some elements (possibly none) of the sequence, and replace each with its length (as a numeric string).
    Concatenate the sequence as the encoded string.

For example, one way to encode an original string "abcdefghijklmnop" might be:

    Split it as a sequence: ["ab", "cdefghijklmn", "o", "p"].
    Choose the second and third elements to be replaced by their lengths, respectively. The sequence becomes ["ab", "12", "1", "p"].
    Concatenate the elements of the sequence to get the encoded string: "ab121p".

Given two encoded strings s1 and s2, consisting of lowercase English letters and digits 1-9 (inclusive), return true if there exists an original string that could be encoded as both s1 and s2. Otherwise, return false.

Note: The test cases are generated such that the number of consecutive digits in s1 and s2 does not exceed 3.

Example 1:

Input: s1 = "internationalization", s2 = "i18n"
Output: true
Explanation: It is possible that "internationalization" was the original string.
- "internationalization"
  -> Split:       ["internationalization"]
  -> Do not replace any element
  -> Concatenate:  "internationalization", which is s1.
- "internationalization"
  -> Split:       ["i", "nternationalizatio", "n"]
  -> Replace:     ["i", "18",                 "n"]
  -> Concatenate:  "i18n", which is s2

Example 2:

Input: s1 = "l123e", s2 = "44"
Output: true
Explanation: It is possible that "leetcode" was the original string.
- "leetcode"
  -> Split:      ["l", "e", "et", "cod", "e"]
  -> Replace:    ["l", "1", "2",  "3",   "e"]
  -> Concatenate: "l123e", which is s1.
- "leetcode"
  -> Split:      ["leet", "code"]
  -> Replace:    ["4",    "4"]
  -> Concatenate: "44", which is s2.

Example 3:

Input: s1 = "a5b", s2 = "c5b"
Output: false
Explanation: It is impossible.
- The original string encoded as s1 must start with the letter 'a'.
- The original string encoded as s2 must start with the letter 'c'.

Constraints:

    1 <= s1.length, s2.length <= 40
    s1 and s2 consist of digits 1-9 (inclusive), and lowercase English letters only.
    The number of consecutive digits in s1 and s2 does not exceed 3.
*/

struct Solution;

impl Solution {
    pub fn possibly_equals(s1: String, s2: String) -> bool {
        let len1: usize = s1.len();
        let chs1: Vec<char> = s1.chars().collect();
        let len2: usize = s2.len();
        let chs2: Vec<char> = s2.chars().collect();
        let mut memo: Vec<Vec<Vec<Option<bool>>>> = vec![vec![vec![None; 2000]; len2 + 1]; len1 + 1];
        Self::dfs(0, 0, 0, &chs1, &chs2, &mut memo)
    }
    fn dfs(
        idx1: usize,
        idx2: usize,
        diff: i16,
        chs1: &Vec<char>,
        chs2: &Vec<char>,
        memo: &mut Vec<Vec<Vec<Option<bool>>>>,
    ) -> bool {
        const RADIX: u32 = 10;
        let len1: usize = chs1.len();
        let len2: usize = chs2.len();
        if idx1 == len1 && idx2 == len2 {
            return diff == 0;
        }
        if let Some(m) = memo[idx1][idx2][(diff + 1000) as usize] {
            return m;
        }
        if idx1 < len1
            && idx2 < len2
            && diff == 0
            && chs1[idx1] == chs2[idx2]
            && Self::dfs(idx1 + 1, idx2 + 1, 0, chs1, chs2, memo)
        {
            memo[idx1][idx2][1000] = Some(true);
            return true;
        }
        if idx1 < len1
            && !chs1[idx1].is_digit(RADIX)
            && diff > 0
            && Self::dfs(idx1 + 1, idx2, diff - 1, chs1, chs2, memo)
        {
            memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
            return true;
        }
        if idx2 < len2
            && !chs2[idx2].is_digit(RADIX)
            && diff < 0
            && Self::dfs(idx1, idx2 + 1, diff + 1, chs1, chs2, memo)
        {
            memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
            return true;
        }
        let mut num1: i16 = 0;
        for idx in idx1..len1 {
            if !chs1[idx].is_digit(RADIX) {
                break;
            }
            num1 = (num1 * 10) + (chs1[idx] as i16 - '0' as i16);
            if Self::dfs(idx + 1, idx2, diff - num1, chs1, chs2, memo) {
                memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
                return true;
            }
        }
        let mut num2: i16 = 0;
        for idx in idx2..len2 {
            if !chs2[idx].is_digit(RADIX) {
                break;
            }
            num2 = (num2 * 10) + (chs2[idx] as i16 - '0' as i16);
            if Self::dfs(idx1, idx + 1, diff + num2, chs1, chs2, memo) {
                memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
                return true;
            }
        }
        memo[idx1][idx2][(diff + 1000) as usize] = Some(false);
        false
    }
}

#[test]
fn test() {
    let cases = vec![
        ("internationalization", "i18n", true),
        ("l123e", "44", true),
        ("a5b", "c5b", false),
        ("a", "2", false),
    ];
    for (s1, s2, expected) in cases {
        assert_eq!(Solution::possibly_equals(s1.to_string(), s2.to_string()), expected);
    }
}
