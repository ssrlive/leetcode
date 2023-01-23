#![allow(dead_code)]

// 1278. Palindrome Partitioning III
// https://leetcode.com/problems/palindrome-partitioning-iii/
// https://leetcode.cn/problems/palindrome-partitioning-iii/
//
// Hard
//
// You are given a string s containing lowercase letters and an integer k. You need to :
//
//     First, change some characters of s to other lowercase English letters.
//     Then divide s into k non-empty disjoint substrings such that each substring is a palindrome.
//
// Return the minimal number of characters that you need to change to divide the string.
//
// Example 1:
//
// Input: s = "abc", k = 2
// Output: 1
// Explanation: You can split the string into "ab" and "c", and change 1 character in "ab" to make it palindrome.
//
// Example 2:
//
// Input: s = "aabbc", k = 3
// Output: 0
// Explanation: You can split the string into "aa", "bb" and "c", all of them are palindrome.
//
// Example 3:
//
// Input: s = "leetcode", k = 8
// Output: 0
//
// Constraints:
//
// -    1 <= k <= s.length <= 100.
// -    s only contains lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let c: Vec<char> = s.chars().collect();
        fn dfs(k: i32, l: usize, c: &Vec<char>, memo: &mut HashMap<(i32, usize), i32>) -> i32 {
            if k as usize >= l {
                return 0;
            }
            if k == 0 && l > 0 {
                return l as i32;
            }
            if let Some(r) = memo.get(&(k, l)) {
                return *r;
            }
            let mut ret = l as i32;
            for ll in (k - 1) as usize..l {
                let mut ri = l - 1;
                let mut li = ll;
                let mut lc = 0;
                while li < ri {
                    if c[li] != c[ri] {
                        lc += 1;
                    }
                    li += 1;
                    ri -= 1;
                }
                ret = ret.min(dfs(k - 1, ll, c, memo) + lc);
            }
            memo.insert((k, l), ret);
            ret
        }

        let mut memo: HashMap<(i32, usize), i32> = HashMap::new();
        dfs(k, c.len(), &c, &mut memo)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::palindrome_partition("abc".to_string(), 2), 1);
    assert_eq!(Solution::palindrome_partition("aabbc".to_string(), 3), 0);
    assert_eq!(Solution::palindrome_partition("leetcode".to_string(), 8), 0);
}
