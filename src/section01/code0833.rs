#![allow(dead_code)]

// 833. Find And Replace in String
// https://leetcode.com/problems/find-and-replace-in-string/
// https://leetcode.cn/problems/find-and-replace-in-string/
//
// You are given a 0-indexed string s that you must perform k replacement operations on. The replacement operations
// are given as three 0-indexed parallel arrays, indices, sources, and targets, all of length k.
//
// To complete the ith replacement operation:
//
// Check if the substring sources[i] occurs at index indices[i] in the original string s.
// If it does not occur, do nothing.
// Otherwise if it does occur, replace that substring with targets[i].
// For example, if s = "abcd", indices[i] = 0, sources[i] = "ab", and targets[i] = "eee", then the result of this replacement will be "eeecd".
//
// All replacement operations must occur simultaneously, meaning the replacement operations should not
// affect the indexing of each other. The testcases will be generated such that the replacements will not overlap.
//
// - For example, a testcase with s = "abc", indices = [0, 1], and sources = ["ab","bc"] will not be generated because the "ab" and "bc" replacements overlap.
//
// Return the resulting string after performing all replacement operations on s.
//
// A substring is a contiguous sequence of characters in a string.
//
// Example 1:
//
// Input: s = "abcd", indices = [0, 2], sources = ["a", "cd"], targets = ["eee", "ffff"]
// Output: "eeebffff"
// Explanation:
// "a" occurs at index 0 in s, so we replace it with "eee".
// "cd" occurs at index 2 in s, so we replace it with "ffff".
//
// Example 2:
//
// Input: s = "abcd", indices = [0, 2], sources = ["ab","ec"], targets = ["eee","ffff"]
// Output: "eeecd"
// Explanation:
// "ab" occurs at index 0 in s, so we replace it with "eee".
// "ec" does not occur at index 2 in s, so we do nothing.
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - k == indices.length == sources.length == targets.length
// - 1 <= k <= 100
// - 0 <= indexes[i] < s.length
// - 1 <= sources[i].length, targets[i].length <= 50
// - s consists of only lowercase English letters.
// - sources[i] and targets[i] consist of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
        let s: Vec<char> = s.chars().collect();

        let mut idxs: Vec<(usize, usize, usize)> = indices
            .into_iter()
            .enumerate()
            .zip(sources)
            .map(|((i, idx), src)| (i, idx as usize, src))
            .filter(|(_, idx, src)| (s.len() - *idx) >= src.len() && s.iter().skip(*idx).zip(src.chars()).all(|(&ch, src_ch)| src_ch == ch))
            .map(|(i, idx, src)| (i, idx, src.len()))
            .collect();

        idxs.sort_by_key(|&(_, idx, _)| idx);

        let start: Box<dyn Iterator<Item = char>> = Box::new(std::iter::empty());

        let (it, last_idx) = idxs.into_iter().fold((start, 0), |(iter_acc, s_next), (i, idx, lsrc)| {
            (
                Box::new(iter_acc.chain(s[s_next..idx].iter().cloned()).chain(targets[i].chars())),
                idx + lsrc,
            )
        });

        let it = if last_idx < s.len() {
            Box::new(it.chain(s[last_idx..].iter().cloned()))
        } else {
            it
        };

        it.collect()
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let indices = vec![0, 2];
    let sources = vec!["a".to_string(), "cd".to_string()];
    let targets = vec!["eee".to_string(), "ffff".to_string()];
    let result = "eeebffff".to_string();
    assert_eq!(Solution::find_replace_string(s, indices, sources, targets), result);

    let s = "abcd".to_string();
    let indices = vec![0, 2];
    let sources = vec!["ab".to_string(), "ec".to_string()];
    let targets = vec!["eee".to_string(), "ffff".to_string()];
    let result = "eeecd".to_string();
    assert_eq!(Solution::find_replace_string(s, indices, sources, targets), result);
}
