#![allow(dead_code)]

// 466. Count The Repetitions
// https://leetcode.com/problems/count-the-repetitions/
//
// We define str = [s, n] as the string str which consists of the string s concatenated n times.
//
// For example, str == ["abc", 3] =="abcabcabc".
// We define that string s1 can be obtained from string s2 if we can remove some characters from s2 such that it becomes s1.
//
// For example, s1 = "abc" can be obtained from s2 = "abdbec" based on our definition by removing the bolded underlined characters.
// You are given two strings s1 and s2 and two integers n1 and n2. You have the two strings str1 = [s1, n1] and str2 = [s2, n2].
//
// Return the maximum integer m such that str = [str2, m] can be obtained from str1.
//
// Example 1:
//
// Input: s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
// Output: 2
//
// Example 2:
//
// Input: s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
// Output: 1
//
// Constraints:
//
// - 1 <= s1.length, s2.length <= 100
// - s1 and s2 consist of lowercase English letters.
// - 1 <= n1, n2 <= 106
//

struct Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let m = s1.len();
        let n = s2.len();
        let mut x = vec![vec![0; n]; m];
        let mut y = vec![vec![0; n]; m];
        let (mut i, mut j) = (0, 1);
        loop {
            i += 1;
            let ii = (i % m as i32) as usize;
            let jj = (j % n as i32) as usize;
            let xx = x[ii][jj];
            let yy = y[ii][jj];
            x[ii][jj] = i;
            y[ii][jj] = j;
            if xx > 0 {
                let k = (n1 * m as i32 - i) / (i - xx);
                i += k * (i - xx);
                j += k * (j - yy);
            }
            if i == n1 * m as i32 {
                return j / (n * n2 as usize) as i32;
            }
            if s1.chars().nth(ii) == s2.chars().nth(jj) {
                j += 1;
            }
        }
    }
}

#[test]
fn test_get_max_repetitions() {
    assert_eq!(
        Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2),
        2
    );
    assert_eq!(
        Solution::get_max_repetitions("acb".to_string(), 1, "acb".to_string(), 1),
        1
    );
}
