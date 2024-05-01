#![allow(dead_code)]

// 3133. Minimum Array End
// https://leetcode.com/problems/minimum-array-end/
// https://leetcode.cn/problems/minimum-array-end/
//
// Medium
//
// You are given two integers n and x. You have to construct an array of positive integers nums of size n where for every
// 0 <= i < n - 1, nums[i + 1] is greater than nums[i], and the result of the bitwise AND operation between all elements of nums is x.
//
// Return the minimum possible value of nums[n - 1].
//
// Example 1:
//
// Input: n = 3, x = 4
//
// Output: 6
//
// Explanation:
//
// nums can be [4,5,6] and its last element is 6.
//
// Example 2:
//
// Input: n = 2, x = 7
//
// Output: 15
//
// Explanation:
//
// nums can be [7,15] and its last element is 15.
//
// Constraints:
//
// 1 <= n, x <= 10^8
//

struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut ans = 0;
        let mut bit: i64;
        let nn = n - 1;
        let mut a = vec![0; 64];
        let mut b = vec![0; 64];
        let x = x as i64;
        let n = nn as i64;

        for i in 0..64 {
            bit = (x >> i) & 1;
            a[i] = bit;

            bit = (n >> i) & 1;
            b[i] = bit;
        }

        let mut pa = 0;
        let mut pb = 0;

        while pa < 63 {
            while a[pa] != 0 && pa < 63 {
                pa += 1;
            }

            a[pa] = b[pb];
            pa += 1;
            pb += 1;
        }

        for (i, &a_i) in a.iter().enumerate().take(64) {
            if a_i == 1 {
                ans += 1 << i;
            }
        }

        ans
    }
}

#[test]
fn test() {
    let n = 3;
    let x = 4;
    let output = 6;
    assert_eq!(Solution::min_end(n, x), output);

    let n = 2;
    let x = 7;
    let output = 15;
    assert_eq!(Solution::min_end(n, x), output);
}
