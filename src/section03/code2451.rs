#![allow(dead_code)]

// 2451. Odd String Difference
// https://leetcode.com/problems/odd-string-difference/
// https://leetcode.cn/problems/odd-string-difference/
//
// You are given an array of equal-length strings words. Assume that the length of each string is n.
//
// Each string words[i] can be converted into a difference integer array difference[i] of length n - 1
// where difference[i][j] = words[i][j+1] - words[i][j] where 0 <= j <= n - 2.
// Note that the difference between two letters is the difference between their positions
// in the alphabet i.e. the position of 'a' is 0, 'b' is 1, and 'z' is 25.
//
// For example, for the string "acb", the difference integer array is [2 - 0, 1 - 2] = [2, -1].
// All the strings in words have the same difference integer array, except one. You should find that string.
//
// Return the string in words that has different difference integer array.
//
// Example 1:
//
// Input: words = ["adc","wzy","abc"]
// Output: "abc"
// Explanation:
// - The difference integer array of "adc" is [3 - 0, 2 - 3] = [3, -1].
// - The difference integer array of "wzy" is [25 - 22, 24 - 25]= [3, -1].
// - The difference integer array of "abc" is [1 - 0, 2 - 1] = [1, 1].
// The odd array out is [1, 1], so we return the corresponding string, "abc".
//
// Example 2:
//
// Input: words = ["aaa","bob","ccc","ddd"]
// Output: "bob"
// Explanation: All the integer arrays are [0, 0] except for "bob", which corresponds to [13, -13].
//
// Constraints:
//
// - 3 <= words.length <= 100
// - n == words[i].length
// - 2 <= n <= 20
// - words[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let get_diff_vec = |s: &str| -> Vec<i32> {
            let f = |(b, a): (u8, u8)| b as i32 - a as i32;
            s[1..].bytes().zip(s.bytes()).map(f).collect()
        };

        let mut map = std::collections::HashMap::with_capacity(2);
        for word in words.into_iter() {
            let key = get_diff_vec(word.as_str());
            let (count, _) = map.entry(key).or_insert((0, word));

            *count += 1;
            if *count > 1 && map.len() > 1 {
                break;
            }
        }

        let f = |(count, word): (i32, String)| (count == 1).then_some(word);
        map.into_values().find_map(f).unwrap_or_default()
    }
}

#[test]
fn test() {
    let words = ["adc", "wzy", "abc"];
    let words = words.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::odd_string(words), "abc".to_string());

    let words = ["aaa", "bob", "ccc", "ddd"];
    let words = words.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::odd_string(words), "bob".to_string());
}
