#![allow(dead_code)]

/*

// 1639. Number of Ways to Form a Target String Given a Dictionary
// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/
// https://leetcode.cn/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/
//
// Hard
//
// You are given a list of strings of the same length words and a string target.

Your task is to form target using the given words under the following rules:

    target should be formed from left to right.
    To form the ith character (0-indexed) of target, you can choose the kth character of the jth string in words if target[i] = words[j][k].
    Once you use the kth character of the jth string of words, you can no longer use the xth character of any string in words where x <= k. In other words, all characters to the left of or at index k become unusuable for every string.
    Repeat the process until you form the string target.

Notice that you can use multiple characters from the same string in words provided the conditions above are met.

Return the number of ways to form target from words. Since the answer may be too large, return it modulo 109 + 7.

Example 1:

Input: words = ["acca","bbbb","caca"], target = "aba"
Output: 6
Explanation: There are 6 ways to form target.
"aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("caca")
"aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("caca")
"aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("acca")
"aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("acca")
"aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("acca")
"aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("caca")

Example 2:

Input: words = ["abba","baab"], target = "bab"
Output: 4
Explanation: There are 4 ways to form target.
"bab" -> index 0 ("baab"), index 1 ("baab"), index 2 ("abba")
"bab" -> index 0 ("baab"), index 1 ("baab"), index 3 ("baab")
"bab" -> index 0 ("baab"), index 2 ("baab"), index 3 ("baab")
"bab" -> index 1 ("abba"), index 2 ("baab"), index 3 ("baab")

Constraints:

    1 <= words.length <= 1000
    1 <= words[i].length <= 1000
    All strings in words have the same length.
    1 <= target.length <= 1000
    words[i] and target contain only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let target = target.as_bytes();
        let n = target.len();
        let mod_num = 1_000_000_007_i64;
        let mut res = vec![0; n + 1];
        res[0] = 1;
        for i in 0..words[0].len() {
            let mut count = vec![0; 26];
            for w in &words {
                count[(w.as_bytes()[i] - b'a') as usize] += 1;
            }
            for j in (0..n).rev() {
                res[j + 1] += res[j] * count[(target[j] - b'a') as usize] % mod_num;
            }
        }
        (res[n] % mod_num) as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["acca", "bbbb", "caca"], "aba", 6),
        (vec!["abba", "baab"], "bab", 4),
    ];
    for (words, target, expected) in cases {
        let words = words.iter().map(|s| s.to_string()).collect();
        let target = target.to_string();
        let actual = Solution::num_ways(words, target);
        assert_eq!(actual, expected);
    }
}
