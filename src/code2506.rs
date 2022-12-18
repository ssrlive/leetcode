#![allow(dead_code)]

// 2506. Count Pairs Of Similar Strings
// https://leetcode.com/problems/count-pairs-of-similar-strings/
// https://leetcode.cn/problems/count-pairs-of-similar-strings/
//
// You are given a 0-indexed string array words.
//
// Two strings are similar if they consist of the same characters.
//
// For example, "abca" and "cba" are similar since both consist of characters 'a', 'b', and 'c'.
// However, "abacba" and "bcfd" are not similar since they do not consist of the same characters.
// Return the number of pairs (i, j) such that 0 <= i < j <= word.length - 1 and the two strings words[i] and words[j] are similar.
//
// Example 1:
//
// Input: words = ["aba","aabb","abcd","bac","aabc"]
// Output: 2
// Explanation: There are 2 pairs that satisfy the conditions:
// - i = 0 and j = 1 : both words[0] and words[1] only consist of characters 'a' and 'b'.
// - i = 3 and j = 4 : both words[3] and words[4] only consist of characters 'a', 'b', and 'c'.
//
// Example 2:
//
// Input: words = ["aabb","ab","ba"]
// Output: 3
// Explanation: There are 3 pairs that satisfy the conditions:
// - i = 0 and j = 1 : both words[0] and words[1] only consist of characters 'a' and 'b'.
// - i = 0 and j = 2 : both words[0] and words[2] only consist of characters 'a' and 'b'.
// - i = 1 and j = 2 : both words[1] and words[2] only consist of characters 'a' and 'b'.
//
// Example 3:
//
// Input: words = ["nba","cba","dba"]
// Output: 0
// Explanation: Since there does not exist any pair that satisfies the conditions, we return 0.
//
// Constraints:
//
// - 1 <= words.length <= 100
// - 1 <= words[i].length <= 100
// - words[i] consist of only lowercase English letters.
//

/*
// cpp solution1
int similarPairs(vector<string>& words) {
    unordered_map<int, int> cnt;
    int res = 0;
    for (string &w: words) {
        int v = 0;
        for (char &c: w) {
            v |= 1 << (c - 'a');
        }
        res += cnt[v]++;
    }
    return res;
}
*/

struct Solution;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut res = 0;
        for w in words {
            let mut v = 0;
            for c in w.chars() {
                v |= 1 << (c as u8 - b'a');
            }
            res += *cnt.entry(v).or_insert(0);
            *cnt.entry(v).or_insert(0) += 1;
        }
        res
    }
}

#[test]
fn test() {
    let words = vec!["aba", "aabb", "abcd", "bac", "aabc"];
    let words = words.into_iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::similar_pairs(words), 2);

    let words = vec!["aabb".to_string(), "ab".to_string(), "ba".to_string()];
    assert_eq!(Solution::similar_pairs(words), 3);

    let words = vec!["nba".to_string(), "cba".to_string(), "dba".to_string()];
    assert_eq!(Solution::similar_pairs(words), 0);
}
