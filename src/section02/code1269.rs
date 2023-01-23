#![allow(dead_code)]

// 1269. Number of Ways to Stay in the Same Place After Some Steps
// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/
// https://leetcode.cn/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/
//
// Hard
//
// You have a pointer at index 0 in an array of size arrLen. At each step, you can move 1 position to the left,
// 1 position to the right in the array, or stay in the same place (The pointer should not be placed outside the array at any time).
//
// Given two integers steps and arrLen, return the number of ways such that your pointer is still
// at index 0 after exactly steps steps. Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: steps = 3, arrLen = 2
// Output: 4
// Explanation: There are 4 differents ways to stay at index 0 after 3 steps.
// Right, Left, Stay
// Stay, Right, Left
// Right, Stay, Left
// Stay, Stay, Stay
//
// Example 2:
//
// Input: steps = 2, arrLen = 4
// Output: 2
// Explanation: There are 2 differents ways to stay at index 0 after 2 steps
// Right, Left
// Stay, Stay
//
// Example 3:
//
// Input: steps = 4, arrLen = 2
// Output: 8
//
// Constraints:
//
// -    1 <= steps <= 500
// -    1 <= arrLen <= 10^6
//

struct Solution;

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let (steps, arr_len) = (steps as usize, arr_len as usize);
        let sz = (steps / 2 + 1).min(arr_len);
        let mut v1 = vec![0; sz + 2];
        let mut v2 = vec![0; sz + 2];
        v1[1] = 1_i64;
        for _ in 0..steps {
            for i in 1..=sz {
                v2[i] = (v1[i] + v1[i - 1] + v1[i + 1]) % 1000000007;
            }
            std::mem::swap(&mut v1, &mut v2);
        }
        v1[1] as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_ways(3, 2), 4);
    assert_eq!(Solution::num_ways(2, 4), 2);
    assert_eq!(Solution::num_ways(4, 2), 8);
}
