#![allow(dead_code)]

// 1324. Print Words Vertically
// https://leetcode.com/problems/print-words-vertically/
// https://leetcode.cn/problems/print-words-vertically/
//
// Medium
//
// Given a string s. Return all the words vertically in the same order in which they appear in s.
// Words are returned as a list of strings, complete with spaces when is necessary. (Trailing spaces are not allowed).
// Each word would be put on only one column and that in one column there will be only one word.
//
// Example 1:
//
// Input: s = "HOW ARE YOU"
// Output: ["HAY","ORO","WEU"]
// Explanation: Each word is printed vertically.
// "HAY"
// "ORO"
// "WEU"
//
// Example 2:
//
// Input: s = "TO BE OR NOT TO BE"
// Output: ["TBONTB","OEROOE","   T"]
// Explanation: Trailing spaces is not allowed.
// "TBONTB"
// "OEROOE"
// "   T"
//
// Example 3:
//
// Input: s = "CONTEST IS COMING"
// Output: ["CIC","OSO","N M","T I","E N","S G","T"]
//
// Constraints:
//
// -    1 <= s.length <= 200
// -    s contains only upper case English letters.
// -    It's guaranteed that there is only one space between 2 words.
//

struct Solution;

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let words = s.split_whitespace().collect::<Vec<&str>>();
        let mut max_len = 0;
        for word in &words {
            if word.len() > max_len {
                max_len = word.len();
            }
        }
        let mut result = vec![];
        for i in 0..max_len {
            let mut line = String::new();
            for word in &words {
                if i < word.len() {
                    line.push(word.chars().nth(i).unwrap());
                } else {
                    line.push(' ');
                }
            }
            result.push(line.trim_end().to_string());
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        ("HOW ARE YOU", vec!["HAY", "ORO", "WEU"]),
        ("TO BE OR NOT TO BE", vec!["TBONTB", "OEROOE", "   T"]),
        ("CONTEST IS COMING", vec!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::print_vertically(s.to_string()), expected);
    }
}
