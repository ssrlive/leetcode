#![allow(dead_code)]

// 2707. Extra Characters in a String
// https://leetcode.com/problems/extra-characters-in-a-string/
// https://leetcode.cn/problems/extra-characters-in-a-string/
//
// Medium
//
// You are given a 0-indexed string s and a dictionary of words dictionary.
// You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary.
// There may be some extra characters in s which are not present in any of the substrings.
//
// Return the minimum number of extra characters left over if you break up s optimally.
//
// Example 1:
//
// Input: s = "leetscode", dictionary = ["leet","code","leetcode"]
// Output: 1
// Explanation: We can break s in two substrings: "leet" from index 0 to 3 and "code" from index 5 to 8.
// There is only 1 unused character (at index 4), so we return 1.
//
// Example 2:
//
// Input: s = "sayhelloworld", dictionary = ["hello","world"]
// Output: 3
// Explanation: We can break s in two substrings: "hello" from index 3 to 7 and "world" from index 8 to 12.
// The characters at indices 0, 1, 2 are not used in any substring and thus are considered as extra characters. Hence, we return 3.
//
// Constraints:
//
//     1 <= s.length <= 50
//     1 <= dictionary.length <= 50
//     1 <= dictionary[i].length <= 50
//     dictionary[i] and s consists of only lowercase English letters
//     dictionary contains distinct words
//

struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dp = vec![0; s.len() + 1];

        for i in (0..s.len()).rev() {
            dp[i] = dp[i + 1] + 1;
            for ss in &dictionary {
                if i + ss.len() <= s.len() && s[i..i + ss.len()] == ss[..] {
                    dp[i] = dp[i].min(dp[i + ss.len()]);
                }
            }
        }
        dp[0]
    }
}

#[test]
fn test() {
    let s = "leetscode".to_string();
    let dictionary = vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()];
    assert_eq!(Solution::min_extra_char(s, dictionary), 1);

    let s = "sayhelloworld".to_string();
    let dictionary = vec!["hello".to_string(), "world".to_string()];
    assert_eq!(Solution::min_extra_char(s, dictionary), 3);
}
