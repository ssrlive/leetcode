#![allow(dead_code)]

// 3144. Minimum Substring Partition of Equal Character Frequency
// https://leetcode.com/problems/minimum-substring-partition-of-equal-character-frequency/
// https://leetcode.cn/problems/minimum-substring-partition-of-equal-character-frequency/
//
// Medium
//
// Given a string s, you need to partition it into one or more balanced substrings.
// For example, if s == "ababcc" then ("abab", "c", "c"), ("ab", "abc", "c"), and ("ababcc")
// are all valid partitions, but ("a", "bab", "cc"), ("aba", "bc", "c"), and ("ab", "abcc") are not.
// The unbalanced substrings are bolded.
//
// Return the minimum number of substrings that you can partition s into.
//
// Note: A balanced string is a string where each character in the string occurs the same number of times.
//
// Example 1:
//
// Input: s = "fabccddg"
//
// Output: 3
//
// Explanation:
//
// We can partition the string s into 3 substrings in one of the following ways: ("fab, "ccdd", "g"), or ("fabc", "cd", "dg").
//
// Example 2:
//
// Input: s = "abababaccddb"
//
// Output: 2
//
// Explanation:
//
// We can partition the string s into 2 substrings like so: ("abab", "abaccddb").
//
// Constraints:
//
// 1 <= s.length <= 1000
// s consists only of English lowercase letters.
//

struct Solution;

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let mut dp = vec![-1; 1001];
        Self::helper(&mut dp, s.len() as i32 - 1, &s)
    }

    fn helper(dp: &mut Vec<i32>, i: i32, s: &String) -> i32 {
        if i < 0 {
            return 0;
        }

        if dp[i as usize] != -1 {
            return dp[i as usize];
        }

        let mut freq = [0; 26];
        let mut res = 1001;
        for j in (0..=i).rev() {
            freq[s.as_bytes()[j as usize] as usize - b'a' as usize] += 1;
            let mut mini = 1001;
            let mut maxi = 0;
            for &freq_k in freq.iter().take(26) {
                if freq_k > 0 {
                    mini = mini.min(freq_k);
                    maxi = maxi.max(freq_k);
                }
            }

            if mini == maxi {
                res = res.min(1 + Self::helper(dp, j - 1, s));
            }
        }

        dp[i as usize] = res;
        res
    }
}

#[test]
fn test() {
    let s = "fabccddg".to_string();
    let output = 3;
    assert_eq!(Solution::minimum_substrings_in_partition(s), output);

    let s = "abababaccddb".to_string();
    let output = 2;
    assert_eq!(Solution::minimum_substrings_in_partition(s), output);
}
