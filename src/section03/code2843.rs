#![allow(dead_code)]

// 2843. Count Symmetric Integers
// https://leetcode.com/problems/count-symmetric-integers/
// https://leetcode.cn/problems/count-symmetric-integers/
//
// Easy
//
// You are given two positive integers low and high.
//
// An integer x consisting of 2 * n digits is symmetric if the sum of the first n digits of x is equal
// to the sum of the last n digits of x. Numbers with an odd number of digits are never symmetric.
//
// Return the number of symmetric integers in the range [low, high].
//
// Example 1:
//
// Input: low = 1, high = 100
// Output: 9
// Explanation: There are 9 symmetric integers between 1 and 100: 11, 22, 33, 44, 55, 66, 77, 88, and 99.
//
// Example 2:
//
// Input: low = 1200, high = 1230
// Output: 4
// Explanation: There are 4 symmetric integers between 1200 and 1230: 1203, 1212, 1221, and 1230.
//
// Constraints:
//
//     1 <= low <= high <= 10^4
//

struct Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high).filter(Self::is_symmetric).count() as _
    }

    fn is_symmetric(num: &i32) -> bool {
        let s = format!("{num}");

        if !s.len().is_multiple_of(2) {
            return false;
        }

        let mid = s.len() / 2;
        let s1 = s.get(0..mid).unwrap();
        let s2 = s.get(mid..).unwrap();

        s1.chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>() == s2.chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_symmetric_integers(1, 100), 9);
    assert_eq!(Solution::count_symmetric_integers(1200, 1230), 4);
}
