#![allow(dead_code)]

// 1160. Find Words That Can Be Formed by Characters
// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
// https://leetcode.cn/problems/find-words-that-can-be-formed-by-characters/
//
// You are given an array of strings words and a string chars.
//
// A string is good if it can be formed by characters from chars (each character can only be used once).
//
// Return the sum of lengths of all good strings in words.
//
// Example 1:
//
// Input: words = ["cat","bt","hat","tree"], chars = "atach"
// Output: 6
// Explanation: The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.
//
// Example 2:
//
// Input: words = ["hello","world","leetcode"], chars = "welldonehoneyr"
// Output: 10
// Explanation: The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.
//
// Constraints:
//
// - 1 <= words.length <= 1000
// - 1 <= words[i].length, chars.length <= 100
// - words[i] and chars consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let get_counter = |word: &String| -> [i32; 26] {
            let mut counter = [0; 26];
            word.as_bytes().iter().for_each(|&b| {
                counter[(b - b'a') as usize] += 1;
            });
            counter
        };

        let counter = get_counter(&chars);
        let f = |word: &String| -> i32 {
            let mut counter = counter;
            for b in word.as_bytes() {
                let ind = (b - b'a') as usize;
                if counter[ind] == 0 {
                    return 0;
                }
                counter[ind] -= 1;
            }
            word.len() as i32
        };
        words.iter().map(f).sum()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["cat", "bt", "hat", "tree"], "atach".to_string(), 6),
        (vec!["hello", "world", "leetcode"], "welldonehoneyr".to_string(), 10),
    ];
    for (words, chars, expected) in cases {
        let words = words.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::count_characters(words, chars), expected);
    }
}
