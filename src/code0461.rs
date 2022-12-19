#![allow(dead_code)]

// 461. Hamming Distance
// https://leetcode.com/problems/hamming-distance/
// https://leetcode.cn/problems/hamming-distance/
//
// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.
//
// Given two integers x and y, return the Hamming distance between them.
//
// Example 1:
//
// Input: x = 1, y = 4
// Output: 2
// Explanation:
// 1   (0 0 0 1)
// 4   (0 1 0 0)
//        ↑   ↑
// The above arrows point to positions where the corresponding bits are different.
//
// Example 2:
//
// Input: x = 3, y = 1
// Output: 1
//
// Constraints:
//
// - 0 <= x, y <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut x = x;
        let mut y = y;
        let mut count = 0;
        while x > 0 || y > 0 {
            if x % 2 != y % 2 {
                count += 1;
            }
            x /= 2;
            y /= 2;
        }
        count
    }
}

#[test]
fn test_hamming_distance() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
    assert_eq!(Solution::hamming_distance(3, 1), 1);
}
