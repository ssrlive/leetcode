#![allow(dead_code)]

/*

// 2085. Count Common Words With One Occurrence
// https://leetcode.com/problems/count-common-words-with-one-occurrence/
// https://leetcode.cn/problems/count-common-words-with-one-occurrence/
//
// Easy
//
// Given two string arrays words1 and words2, return the number of strings that appear exactly once in each of the two arrays.

Example 1:

Input: words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
Output: 2
Explanation:
- "leetcode" appears exactly once in each of the two arrays. We count this string.
- "amazing" appears exactly once in each of the two arrays. We count this string.
- "is" appears in each of the two arrays, but there are 2 occurrences of it in words1. We do not count this string.
- "as" appears once in words1, but does not appear in words2. We do not count this string.
Thus, there are 2 strings that appear exactly once in each of the two arrays.

Example 2:

Input: words1 = ["b","bb","bbb"], words2 = ["a","aa","aaa"]
Output: 0
Explanation: There are no strings that appear in each of the two arrays.

Example 3:

Input: words1 = ["a","ab"], words2 = ["a","a","a","ab"]
Output: 1
Explanation: The only string that appears exactly once in each of the two arrays is "ab".

Constraints:

    1 <= words1.length, words2.length <= 1000
    1 <= words1[i].length, words2[j].length <= 30
    words1[i] and words2[j] consists only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let (mut m1, mut m2) = (HashMap::new(), HashMap::new());
        words1.iter().for_each(|w| *m1.entry(w).or_insert(0) += 1);
        words2.iter().for_each(|w| {
            if let Some(v) = m1.get(w)
                && v == &1
            {
                *m2.entry(w).or_insert(0) += 1;
            }
        });
        m2.values().filter(|v| **v == 1).count() as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["leetcode", "is", "amazing", "as", "is"], vec!["amazing", "leetcode", "is"], 2),
        (vec!["b", "bb", "bbb"], vec!["a", "aa", "aaa"], 0),
        (vec!["a", "ab"], vec!["a", "a", "a", "ab"], 1),
    ];
    for (words1, words2, expected) in cases {
        let words1 = words1.iter().map(|s| s.to_string()).collect();
        let words2 = words2.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::count_words(words1, words2), expected);
    }
}
