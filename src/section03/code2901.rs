#![allow(dead_code)]

// 2901. Longest Unequal Adjacent Groups Subsequence II
// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii/
// https://leetcode.cn/problems/longest-unequal-adjacent-groups-subsequence-ii/
//
// Medium
//
// You are given an integer n, a 0-indexed string array words, and a 0-indexed array groups, both arrays having length n.
//
// The hamming distance between two strings of equal length is the number of positions at which the corresponding characters are different.
//
// You need to select the longest subsequence from an array of indices [0, 1, ..., n - 1],
// such that for the subsequence denoted as [i0, i1, ..., ik - 1] having length k, the following holds:
//
// For adjacent indices in the subsequence, their corresponding groups are unequal, i.e., groups[ij] != groups[ij + 1], for each j where 0 < j + 1 < k.
// words[ij] and words[ij + 1] are equal in length, and the hamming distance between them is 1, where 0 < j + 1 < k, for all indices in the subsequence.
// Return a string array containing the words corresponding to the indices (in order) in the selected subsequence. If there are multiple answers, return any of them.
//
// A subsequence of an array is a new array that is formed from the original array
// by deleting some (possibly none) of the elements without disturbing the relative positions of the remaining elements.
//
// Note: strings in words may be unequal in length.
//
// Example 1:
//
// Input: n = 3, words = ["bab","dab","cab"], groups = [1,2,2]
// Output: ["bab","cab"]
// Explanation: A subsequence that can be selected is [0,2].
// - groups[0] != groups[2]
// - words[0].length == words[2].length, and the hamming distance between them is 1.
// So, a valid answer is [words[0],words[2]] = ["bab","cab"].
// Another subsequence that can be selected is [0,1].
// - groups[0] != groups[1]
// - words[0].length == words[1].length, and the hamming distance between them is 1.
// So, another valid answer is [words[0],words[1]] = ["bab","dab"].
// It can be shown that the length of the longest subsequence of indices that satisfies the conditions is 2.
//
// Example 2:
//
// Input: n = 4, words = ["a","b","c","d"], groups = [1,2,3,4]
// Output: ["a","b","c","d"]
// Explanation: We can select the subsequence [0,1,2,3].
// It satisfies both conditions.
// Hence, the answer is [words[0],words[1],words[2],words[3]] = ["a","b","c","d"].
// It has the longest length among all subsequences of indices that satisfy the conditions.
// Hence, it is the only answer.
//
// Constraints:
//
// 1 <= n == words.length == groups.length <= 1000
// 1 <= words[i].length <= 10
// 1 <= groups[i] <= n
// words consists of distinct strings.
// words[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn get_words_in_longest_subsequence(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut dp = vec![1; n as usize];
        let mut prev = vec![0; n as usize];

        for i in 1..n as usize {
            for j in 0..i {
                if dp[i] > dp[j] {
                    continue;
                }
                if groups[i] == groups[j] {
                    continue;
                }
                if !Self::check(&words[i], &words[j]) {
                    continue;
                }
                prev[i] = j;
                dp[i] = dp[j] + 1;
            }
        }
        let mx = *dp.iter().max().unwrap();
        let mut ret = vec![];
        for (i, &dp_i) in dp.iter().enumerate().take(n as usize) {
            if dp_i < mx {
                continue;
            }
            let mut u = i;
            let mut cnt = dp_i;
            while cnt > 0 {
                ret.push(words[u].clone());
                u = prev[u];
                cnt -= 1;
            }
            break;
        }
        ret.reverse();
        ret
    }

    fn check(s: &String, t: &String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut cnt = 0;
        let s = (*s).clone().chars().collect::<Vec<_>>();
        let t = (*t).clone().chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            if s[i] != t[i] {
                cnt += 1;
            }
        }

        cnt == 1
    }
}

#[test]
fn test() {
    let n = 3;
    let words = vec!["bab".to_string(), "dab".to_string(), "cab".to_string()];
    let groups = vec![1, 2, 2];
    let res = [
        vec!["bab".to_string(), "cab".to_string()],
        vec!["bab".to_string(), "dab".to_string()],
    ];
    let r = Solution::get_words_in_longest_subsequence(n, words, groups);
    assert!(res.contains(&r));

    let n = 4;
    let words = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
    let groups = vec![1, 2, 3, 4];
    let res = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
    assert_eq!(Solution::get_words_in_longest_subsequence(n, words, groups), res);
}
