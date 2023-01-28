#![allow(dead_code)]

// 1397. Find All Good Strings
// https://leetcode.com/problems/find-all-good-strings/
// https://leetcode.cn/problems/find-all-good-strings/
//
// Hard
//
// Given the strings s1 and s2 of size n and the string evil, return the number of good strings.
//
// A good string has size n, it is alphabetically greater than or equal to s1, it is alphabetically smaller than or equal to s2,
// and it does not contain the string evil as a substring. Since the answer can be a huge number, return this modulo 109 + 7.
//
// Example 1:
//
// Input: n = 2, s1 = "aa", s2 = "da", evil = "b"
// Output: 51
// Explanation: There are 25 good strings starting with 'a': "aa","ac","ad",...,"az".
// Then there are 25 good strings starting with 'c': "ca","cc","cd",...,"cz" and finally there is one good string starting with 'd': "da".
//
// Example 2:
//
// Input: n = 8, s1 = "leetcode", s2 = "leetgoes", evil = "leet"
// Output: 0
// Explanation: All strings greater than or equal to s1 and smaller than or equal to s2 start with the prefix "leet", therefore, there is not any good string.
//
// Example 3:
//
// Input: n = 2, s1 = "gx", s2 = "gz", evil = "x"
// Output: 2
//
// Constraints:
//
// -    s1.length == n
// -    s2.length == n
// -    s1 <= s2
// -    1 <= n <= 500
// -    1 <= evil.length <= 50
// -    All strings consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let mut dp = vec![vec![vec![vec![0; 2]; 2]; evil.len() + 1]; n as usize + 1];
        let lps = Self::compute_lps(&evil);
        Self::dfs(0, 0, true, true, n, &s1, &s2, &evil, &lps, &mut dp)
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        i: usize,
        evil_matched: usize,
        left_bound: bool,
        right_bound: bool,
        n: i32,
        s1: &String,
        s2: &String,
        evil: &String,
        lps: &Vec<usize>,
        dp: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if evil_matched == evil.len() {
            return 0;
        }
        if i == n as usize {
            return 1;
        }
        if dp[i][evil_matched][left_bound as usize][right_bound as usize] != 0 {
            return dp[i][evil_matched][left_bound as usize][right_bound as usize];
        }
        let from = if left_bound { s1.as_bytes()[i] } else { b'a' };
        let to = if right_bound { s2.as_bytes()[i] } else { b'z' };
        let mut res = 0;
        for c in from..=to {
            let mut j = evil_matched;
            while j > 0 && evil.as_bytes()[j] != c {
                j = lps[j - 1];
            }
            if evil.as_bytes()[j] == c {
                j += 1;
            }
            res += Self::dfs(
                i + 1,
                j,
                left_bound && (c == from),
                right_bound && (c == to),
                n,
                s1,
                s2,
                evil,
                lps,
                dp,
            );
            res %= 1000000007;
        }
        dp[i][evil_matched][left_bound as usize][right_bound as usize] = res;
        res
    }

    fn compute_lps(str: &String) -> Vec<usize> {
        let n = str.len();
        let mut lps = vec![0; n];
        for i in 1..n {
            let mut j = lps[i - 1];
            while j > 0 && str.as_bytes()[i] != str.as_bytes()[j] {
                j = lps[j - 1];
            }
            if str.as_bytes()[i] == str.as_bytes()[j] {
                lps[i] = j + 1;
            }
        }
        lps
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, "aa".to_string(), "da".to_string(), "b".to_string(), 51),
        (8, "leetcode".to_string(), "leetgoes".to_string(), "leet".to_string(), 0),
        (2, "gx".to_string(), "gz".to_string(), "x".to_string(), 2),
    ];
    for (n, s1, s2, evil, expected) in cases {
        assert_eq!(Solution::find_good_strings(n, s1, s2, evil), expected);
    }
}
