#![allow(dead_code)]

// 2559. Count Vowel Strings in Ranges
// https://leetcode.com/problems/count-vowel-strings-in-ranges/
// https://leetcode.cn/problems/count-vowel-strings-in-ranges/
//
// Medium
//
// You are given a 0-indexed array of strings words and a 2D array of integers queries.
//
// Each query queries[i] = [li, ri] asks us to find the number of strings present in the range li to ri
// (both inclusive) of words that start and end with a vowel.
//
// Return an array ans of size queries.length, where ans[i] is the answer to the ith query.
//
// Note that the vowel letters are 'a', 'e', 'i', 'o', and 'u'.
//
// Example 1:
//
// Input: words = ["aba","bcb","ece","aa","e"], queries = [[0,2],[1,4],[1,1]]
// Output: [2,3,0]
// Explanation: The strings starting and ending with a vowel are "aba", "ece", "aa" and "e".
// The answer to the query [0,2] is 2 (strings "aba" and "ece").
// to query [1,4] is 3 (strings "ece", "aa", "e").
// to query [1,1] is 0.
// We return [2,3,0].
//
// Example 2:
//
// Input: words = ["a","e","i"], queries = [[0,2],[0,1],[2,2]]
// Output: [3,2,1]
// Explanation: Every string satisfies the conditions, so we return [3,2,1].
//
// Constraints:
//
// -    1 <= words.length <= 10^5
// -    1 <= words[i].length <= 40
// -    words[i] consists only of lowercase English letters.
// -    sum(words[i].length) <= 3 * 10^5
// -    1 <= queries.length <= 10^5
// -    0 <= li <= ri < words.length
//

struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn _vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Option<Vec<i32>> {
            let vw = vec!['a', 'e', 'i', 'o', 'u'];
            let mut ps = vec![0];
            for w in words {
                let w = w.chars().collect::<Vec<_>>();
                ps.push(ps.last()? + (vw.contains(w.first()?) && vw.contains(w.last()?)) as i32);
            }
            let mut res = vec![];
            for q in queries {
                let v = *ps.get(*q.get(1)? as usize + 1)? - *ps.get(*q.first()? as usize)?;
                res.push(v);
            }
            Some(res)
        }
        _vowel_strings(words, queries).unwrap_or_default()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["aba", "bcb", "ece", "aa", "e"],
            vec![vec![0, 2], vec![1, 4], vec![1, 1]],
            vec![2, 3, 0],
        ),
        (
            vec!["a", "e", "i"],
            vec![vec![0, 2], vec![0, 1], vec![2, 2]],
            vec![3, 2, 1],
        ),
    ];
    for (words, queries, expect) in cases {
        let words = words.into_iter().map(|s| s.to_string()).collect();
        let ans = Solution::vowel_strings(words, queries);
        assert_eq!(ans, expect);
    }
}
