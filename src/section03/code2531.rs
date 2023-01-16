#![allow(dead_code)]

// 2531. Make Number of Distinct Characters Equal
// https://leetcode.com/problems/make-number-of-distinct-characters-equal/
// https://leetcode.cn/problems/make-number-of-distinct-characters-equal/
//
// You are given two 0-indexed strings word1 and word2.
//
// A move consists of choosing two indices i and j such that 0 <= i < word1.length and 0 <= j < word2.length and swapping word1[i] with word2[j].
//
// Return true if it is possible to get the number of distinct characters in word1 and word2 to be equal with exactly one move. Return false otherwise.
//
// Example 1:
//
// Input: word1 = "ac", word2 = "b"
// Output: false
// Explanation: Any pair of swaps would yield two distinct characters in the first string, and one in the second string.
//
// Example 2:
//
// Input: word1 = "abcc", word2 = "aab"
// Output: true
// Explanation: We swap index 2 of the first string with index 0 of the second string.
//   The resulting strings are word1 = "abac" and word2 = "cab", which both have 3 distinct characters.
//
// Example 3:
//
// Input: word1 = "abcde", word2 = "fghij"
// Output: true
// Explanation: Both resulting strings will have 5 distinct characters, regardless of which indices we swap.
//
// Constraints:
//
// - 1 <= word1.length, word2.length <= 10^5
// - word1 and word2 consist of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let (mut a, mut b) = (vec![0; 26], vec![0; 26]);

        for c in word1.chars() {
            a[(c as u8 - b'a') as usize] += 1;
        }
        for c in word2.chars() {
            b[(c as u8 - b'a') as usize] += 1;
        }

        for i in 0..26 {
            for j in 0..26 {
                if a[i] == 0 || b[j] == 0 {
                    continue;
                }

                let (mut c, mut d) = (a.clone(), b.clone());
                c[i] -= 1;
                c[j] += 1;
                d[i] += 1;
                d[j] -= 1;

                let (mut cnt1, mut cnt2) = (0, 0);
                for it in c {
                    if it > 0 {
                        cnt1 += 1;
                    }
                }
                for it in d {
                    if it > 0 {
                        cnt2 += 1;
                    }
                }

                if cnt1 == cnt2 {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_it_possible("ac".to_string(), "b".to_string()), false);
    assert_eq!(Solution::is_it_possible("abcc".to_string(), "aab".to_string()), true);
    assert_eq!(Solution::is_it_possible("abcde".to_string(), "fghij".to_string()), true);
}
