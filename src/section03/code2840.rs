#![allow(dead_code)]

// 2840. Check if Strings Can be Made Equal With Operations II
// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii/
// https://leetcode.cn/problems/check-if-strings-can-be-made-equal-with-operations-ii/
//
// Medium
//
// You are given two strings s1 and s2, both of length n, consisting of lowercase English letters.
//
// You can apply the following operation on any of the two strings any number of times:
//
//     - Choose any two indices i and j such that i < j and the difference j - i is even,
//       then swap the two characters at those indices in the string.
//
// Return true if you can make the strings s1 and s2 equal, and false otherwise.
//
// Example 1:
//
// Input: s1 = "abcdba", s2 = "cabdab"
// Output: true
// Explanation: We can apply the following operations on s1:
// - Choose the indices i = 0, j = 2. The resulting string is s1 = "cbadba".
// - Choose the indices i = 2, j = 4. The resulting string is s1 = "cbbdaa".
// - Choose the indices i = 1, j = 5. The resulting string is s1 = "cabdab" = s2.
//
// Example 2:
//
// Input: s1 = "abe", s2 = "bea"
// Output: false
// Explanation: It is not possible to make the two strings equal.
//
// Constraints:
//
//     n == s1.length == s2.length
//     1 <= n <= 10^5
//     s1 and s2 consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut data = vec![vec![0; 26]; 2];
        let s1 = s1.chars().collect::<Vec<_>>();
        let s2 = s2.chars().collect::<Vec<_>>();

        for i in 0..s1.len() {
            data[i % 2][s1[i] as usize - 'a' as usize] += 1;
            data[i % 2][s2[i] as usize - 'a' as usize] -= 1;
        }

        for item in data.iter().take(2) {
            for &k in item {
                if k != 0 {
                    return false;
                }
            }
        }

        true
    }
}

#[test]
fn test() {
    assert!(Solution::check_strings("abcdba".to_string(), "cabdab".to_string()));
    assert!(!Solution::check_strings("abe".to_string(), "bea".to_string()));
}
