#![allow(dead_code)]

// 3093. Longest Common Suffix Queries
// https://leetcode.com/problems/longest-common-suffix-queries/
// https://leetcode.cn/problems/longest-common-suffix-queries/
//
// Hard
//
// You are given two arrays of strings wordsContainer and wordsQuery.
//
// For each wordsQuery[i], you need to find a string from wordsContainer that has the longest common suffix with wordsQuery[i].
// If there are two or more strings in wordsContainer that share the longest common suffix, find the string that is the smallest in length.
// If there are two or more such strings that have the same smallest length, find the one that occurred earlier in wordsContainer.
//
// Return an array of integers ans, where ans[i] is the index of the string in wordsContainer that has the longest common suffix with wordsQuery[i].
//
// Example 1:
//
// Input: wordsContainer = ["abcd","bcd","xbcd"], wordsQuery = ["cd","bcd","xyz"]
//
// Output: [1,1,1]
//
// Explanation:
//
// Let's look at each wordsQuery[i] separately:
//
// - For wordsQuery[0] = "cd", strings from wordsContainer that share the longest common suffix "cd" are at indices 0, 1, and 2.
//   Among these, the answer is the string at index 1 because it has the shortest length of 3.
// - For wordsQuery[1] = "bcd", strings from wordsContainer that share the longest common suffix "bcd" are at indices 0, 1, and 2.
//   Among these, the answer is the string at index 1 because it has the shortest length of 3.
// - For wordsQuery[2] = "xyz", there is no string from wordsContainer that shares a common suffix.
//   Hence the longest common suffix is "", that is shared with strings at index 0, 1, and 2.
//   Among these, the answer is the string at index 1 because it has the shortest length of 3.
//
// Example 2:
//
// Input: wordsContainer = ["abcdefgh","poiuygh","ghghgh"], wordsQuery = ["gh","acbfgh","acbfegh"]
//
// Output: [2,0,2]
//
// Explanation:
//
// Let's look at each wordsQuery[i] separately:
//
// - For wordsQuery[0] = "gh", strings from wordsContainer that share the longest common suffix "gh" are at indices 0, 1, and 2.
//   Among these, the answer is the string at index 2 because it has the shortest length of 6.
// - For wordsQuery[1] = "acbfgh", only the string at index 0 shares the longest common suffix "fgh".
//   Hence it is the answer, even though the string at index 2 is shorter.
// - For wordsQuery[2] = "acbfegh", strings from wordsContainer that share the longest common suffix "gh" are at indices 0, 1, and 2.
//   Among these, the answer is the string at index 2 because it has the shortest length of 6.
//
// Constraints:
//
//     1 <= wordsContainer.length, wordsQuery.length <= 10^4
//     1 <= wordsContainer[i].length <= 5 * 10^3
//     1 <= wordsQuery[i].length <= 5 * 10^3
//     wordsContainer[i] consists only of lowercase English letters.
//     wordsQuery[i] consists only of lowercase English letters.
//     Sum of wordsContainer[i].length is at most 5 * 10^5.
//     Sum of wordsQuery[i].length is at most 5 * 10^5.
//

struct Solution;

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mod1 = 1_000_000_007;
        let mod2 = 1_000_000_009;

        let mut mp = std::collections::HashMap::<(i64, i64), (i64, i64)>::new();

        let c = words_container.len();
        let q = words_query.len();
        let mut res = vec![-1; q];
        let mut minn = 0;
        let mut mins = i64::MAX;
        for (i, words_container_i) in words_container.iter().enumerate().take(c) {
            let word = &words_container_i;
            let n = word.len() - 1;
            if mins > n as i64 {
                mins = n as i64;
                minn = i as i64;
            }
            let mut hash1 = 0;
            let mut hash2 = 0;
            for j in (0..=n).rev() {
                hash1 = ((hash1 * 31) % mod1 + (word.as_bytes()[j] - b'a' + 1) as i64) % mod1;
                hash2 = ((hash2 * 37) % mod2 + (word.as_bytes()[j] - b'a' + 1) as i64) % mod2;
                if let Some(v) = mp.get_mut(&(hash1, hash2)) {
                    if v.0 > n as i64 {
                        *v = (n as i64, i as i64);
                    }
                } else {
                    mp.insert((hash1, hash2), (n as i64, i as i64));
                }
            }
        }

        for i in 0..q {
            let word = &words_query[i];
            let n = word.len() - 1;
            let mut hash1 = 0;
            let mut hash2 = 0;
            for j in (0..=n).rev() {
                hash1 = ((hash1 * 31) % mod1 + (word.as_bytes()[j] - b'a' + 1) as i64) % mod1;
                hash2 = ((hash2 * 37) % mod2 + (word.as_bytes()[j] - b'a' + 1) as i64) % mod2;
                if let Some(v) = mp.get(&(hash1, hash2)) {
                    res[i] = v.1;
                }
            }

            if res[i] == -1 {
                res[i] = minn;
            }
        }

        res.into_iter().map(|x| x as i32).collect()
    }
}

#[test]
fn test() {
    let words_container = ["abcd", "bcd", "xbcd"].iter().map(|&s| s.to_string()).collect();
    let words_query = ["cd", "bcd", "xyz"].iter().map(|&s| s.to_string()).collect();
    let ans = Solution::string_indices(words_container, words_query);
    assert_eq!(ans, vec![1, 1, 1]);

    let words_container = ["abcdefgh", "poiuygh", "ghghgh"].iter().map(|&s| s.to_string()).collect();
    let words_query = ["gh", "acbfgh", "acbfegh"].iter().map(|&s| s.to_string()).collect();
    let ans = Solution::string_indices(words_container, words_query);
    assert_eq!(ans, vec![2, 0, 2]);
}
