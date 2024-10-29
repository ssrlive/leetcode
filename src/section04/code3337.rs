#![allow(dead_code)]

// 3337. Total Characters in String After Transformations II
// https://leetcode.com/problems/total-characters-in-string-after-transformations-ii/
// https://leetcode.cn/problems/total-characters-in-string-after-transformations-ii/
//
// Hard
//
// You are given a string s consisting of lowercase English letters, an integer t representing the number of transformations to perform,
// and an array nums of size 26. In one transformation, every character in s is replaced according to the following rules:
//
// - Replace s[i] with the next nums[s[i] - 'a'] consecutive characters in the alphabet. For example, if s[i] = 'a' and nums[0] = 3,
//   the character 'a' transforms into the next 3 consecutive characters ahead of it, which results in "bcd".
// - The transformation wraps around the alphabet if it exceeds 'z'. For example, if s[i] = 'y' and nums[24] = 3,
//   the character 'y' transforms into the next 3 consecutive characters ahead of it, which results in "zab".
//
// Return the length of the resulting string after exactly t transformations.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "abcyy", t = 2, nums = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2]
//
// Output: 7
//
// Explanation:
//
//    First Transformation (t = 1):
//        'a' becomes 'b' as nums[0] == 1
//        'b' becomes 'c' as nums[1] == 1
//        'c' becomes 'd' as nums[2] == 1
//        'y' becomes 'z' as nums[24] == 1
//        'y' becomes 'z' as nums[24] == 1
//        String after the first transformation: "bcdzz"
//
//    Second Transformation (t = 2):
//        'b' becomes 'c' as nums[1] == 1
//        'c' becomes 'd' as nums[2] == 1
//        'd' becomes 'e' as nums[3] == 1
//        'z' becomes 'ab' as nums[25] == 2
//        'z' becomes 'ab' as nums[25] == 2
//        String after the second transformation: "cdeabab"
//
//    Final Length of the string: The string is "cdeabab", which has 7 characters.
//
// Example 2:
//
// Input: s = "azbk", t = 1, nums = [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]
//
// Output: 8
//
// Explanation:
//
//    First Transformation (t = 1):
//        'a' becomes 'bc' as nums[0] == 2
//        'z' becomes 'ab' as nums[25] == 2
//        'b' becomes 'cd' as nums[1] == 2
//        'k' becomes 'lm' as nums[10] == 2
//        String after the first transformation: "bcabcdlm"
//
//    Final Length of the string: The string is "bcabcdlm", which has 8 characters.
//
// Constraints:
//
//    1 <= s.length <= 10^5
//    s consists only of lowercase English letters.
//    1 <= t <= 10^9
//    nums.length == 26
//    1 <= nums[i] <= 25
//

struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut mp = vec![0; 26];
        for c in s.chars() {
            mp[c as usize - 'a' as usize] += 1;
        }
        let mut transform = vec![vec![0; 26]; 26];
        for i in 0..26 {
            for j in 1..=nums[i] {
                transform[(i + j as usize) % 26][i] += 1;
            }
        }
        let powered_transform = Self::matrix_expo(transform, t);
        let mut ans = vec![0; 26];
        for i in 0..26 {
            for (j, &mp_j) in mp.iter().enumerate().take(26) {
                ans[i] = (ans[i] + powered_transform[i][j] * mp_j) % MOD;
            }
        }
        let mut sum = 0;
        for &ans_i in ans.iter().take(26) {
            sum = (sum + ans_i) % MOD;
        }
        sum as i32
    }

    fn multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
        let mut result = vec![vec![0; 26]; 26];
        for i in 0..26 {
            for j in 0..26 {
                for (k, b_k) in b.iter().enumerate().take(26) {
                    result[i][j] = (result[i][j] + a[i][k] * b_k[j]) % MOD;
                }
            }
        }
        result
    }

    fn matrix_expo(mut base: Vec<Vec<i64>>, mut exp: i32) -> Vec<Vec<i64>> {
        let mut result = vec![vec![0; 26]; 26];
        for (i, result_i) in result.iter_mut().enumerate().take(26) {
            result_i[i] = 1;
        }
        while exp > 0 {
            if exp % 2 == 1 {
                result = Self::multiply(&result, &base);
            }
            base = Self::multiply(&base, &base);
            exp /= 2;
        }
        result
    }
}

#[test]
fn test() {
    let s = "abcyy".to_string();
    let t = 2;
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2];
    let output = 7;
    assert_eq!(Solution::length_after_transformations(s, t, nums), output);

    let s = "azbk".to_string();
    let t = 1;
    let nums = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
    let output = 8;
    assert_eq!(Solution::length_after_transformations(s, t, nums), output);
}
