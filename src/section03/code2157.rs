#![allow(dead_code)]

/*

// 2157. Groups of Strings
// https://leetcode.com/problems/groups-of-strings/
// https://leetcode.cn/problems/groups-of-strings/
//
// Hard
//
// You are given a 0-indexed array of strings words. Each string consists of lowercase English letters only. No letter occurs more than once in any string of words.

Two strings s1 and s2 are said to be connected if the set of letters of s2 can be obtained from the set of letters of s1 by any one of the following operations:

    Adding exactly one letter to the set of the letters of s1.
    Deleting exactly one letter from the set of the letters of s1.
    Replacing exactly one letter from the set of the letters of s1 with any letter, including itself.

The array words can be divided into one or more non-intersecting groups. A string belongs to a group if any one of the following is true:

    It is connected to at least one other string of the group.
    It is the only string present in the group.

Note that the strings in words should be grouped in such a manner that a string belonging to a group cannot be connected to a string present in any other group. It can be proved that such an arrangement is always unique.

Return an array ans of size 2 where:

    ans[0] is the maximum number of groups words can be divided into, and
    ans[1] is the size of the largest group.

Example 1:

Input: words = ["a","b","ab","cde"]
Output: [2,3]
Explanation:
- words[0] can be used to obtain words[1] (by replacing 'a' with 'b'), and words[2] (by adding 'b'). So words[0] is connected to words[1] and words[2].
- words[1] can be used to obtain words[0] (by replacing 'b' with 'a'), and words[2] (by adding 'a'). So words[1] is connected to words[0] and words[2].
- words[2] can be used to obtain words[0] (by deleting 'b'), and words[1] (by deleting 'a'). So words[2] is connected to words[0] and words[1].
- words[3] is not connected to any string in words.
Thus, words can be divided into 2 groups ["a","b","ab"] and ["cde"]. The size of the largest group is 3.

Example 2:

Input: words = ["a","ab","abc"]
Output: [1,3]
Explanation:
- words[0] is connected to words[1].
- words[1] is connected to words[0] and words[2].
- words[2] is connected to words[1].
Since all strings are connected to each other, they should be grouped together.
Thus, the size of the largest group is 3.

Constraints:

    1 <= words.length <= 2 * 10^4
    1 <= words[i].length <= 26
    words[i] consists of lowercase English letters only.
    No letter occurs more than once in words[i].
*/

struct Solution;

impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;
        fn dfs(mask: i32, m: &mut HashMap<i32, i32>) -> i32 {
            let mut res = 0;
            let it = m.get(&mask);
            if it.is_none() {
                return res;
            }
            res += it.unwrap();
            m.remove(&mask);
            for i in 0..26 {
                res += dfs(mask ^ (1 << i), m);
                for j in (i + 1)..26 {
                    if ((mask >> i) & 1) != ((mask >> j) & 1) {
                        res += dfs(mask ^ (1 << i) ^ (1 << j), m);
                    }
                }
            }
            res
        }

        let mut m = HashMap::new();
        let mut groups = 0;
        let mut max_size = 0;
        for w in words.iter() {
            let mut mask = 0;
            for ch in w.chars() {
                mask |= 1 << (ch as u8 - b'a');
            }
            *m.entry(mask).or_insert(0) += 1;
        }
        while !m.is_empty() {
            let mask = *m.iter().next().unwrap().0;
            let size = dfs(mask, &mut m);
            max_size = max_size.max(size);
            groups += if size > 0 { 1 } else { 0 };
        }
        vec![groups, max_size]
    }
}

#[test]
fn test() {
    let cases = vec![(vec!["a", "b", "ab", "cde"], vec![2, 3]), (vec!["a", "ab", "abc"], vec![1, 3])];
    for (words, expected) in cases {
        let words: Vec<String> = words.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::group_strings(words), expected);
    }
}
