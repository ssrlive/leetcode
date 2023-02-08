#![allow(dead_code)]

/*

// 1492. The kth Factor of n
// https://leetcode.com/problems/the-kth-factor-of-n/
// https://leetcode.cn/problems/the-kth-factor-of-n/
//
// Medium
//
// You are given two positive integers n and k. A factor of an integer n is defined as an integer i where n % i == 0.

Consider a list of all factors of n sorted in ascending order, return the kth factor in this list or return -1 if n has less than k factors.

Example 1:

Input: n = 12, k = 3
Output: 3
Explanation: Factors list is [1, 2, 3, 4, 6, 12], the 3rd factor is 3.

Example 2:

Input: n = 7, k = 2
Output: 7
Explanation: Factors list is [1, 7], the 2nd factor is 7.

Example 3:

Input: n = 4, k = 4
Output: -1
Explanation: Factors list is [1, 2, 4], there is only 3 factors. We should return -1.

Constraints:

    1 <= k <= n <= 1000

Follow up:

Could you solve this problem in less than O(n) complexity?
*/

struct Solution;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut count = 0;
        for i in 1..=n {
            if n % i == 0 {
                count += 1;
                if count == k {
                    return i;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let cases = vec![(12, 3, 3), (7, 2, 7), (4, 4, -1)];
    for (n, k, expected) in cases {
        assert_eq!(Solution::kth_factor(n, k), expected);
    }
}
