#![allow(dead_code)]

// 3361. Shift Distance Between Two Strings
// https://leetcode.com/problems/shift-distance-between-two-strings/
// https://leetcode.cn/problems/shift-distance-between-two-strings/
//
// Medium
//
// You are given two strings s and t of the same length, and two integer arrays nextCost and previousCost.
//
// In one operation, you can pick any index i of s, and perform either one of the following actions:
//
// Shift s[i] to the next letter in the alphabet. If s[i] == 'z', you should replace it with 'a'. This operation costs nextCost[j] where j is the index of s[i] in the alphabet.
// Shift s[i] to the previous letter in the alphabet. If s[i] == 'a', you should replace it with 'z'. This operation costs previousCost[j] where j is the index of s[i] in the alphabet.
// The shift distance is the minimum total cost of operations required to transform s into t.
//
// Return the shift distance from s to t.
//
// Example 1:
//
// Input: s = "abab", t = "baba", nextCost = [100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], previousCost = [1,100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
//
// Output: 2
//
// Explanation:
//
// We choose index i = 0 and shift s[0] 25 times to the previous character for a total cost of 1.
// We choose index i = 1 and shift s[1] 25 times to the next character for a total cost of 0.
// We choose index i = 2 and shift s[2] 25 times to the previous character for a total cost of 1.
// We choose index i = 3 and shift s[3] 25 times to the next character for a total cost of 0.
//
// Example 2:
//
// Input: s = "leet", t = "code", nextCost = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], previousCost = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
//
// Output: 31
//
// Explanation:
//
// We choose index i = 0 and shift s[0] 9 times to the previous character for a total cost of 9.
// We choose index i = 1 and shift s[1] 10 times to the next character for a total cost of 10.
// We choose index i = 2 and shift s[2] 1 time to the previous character for a total cost of 1.
// We choose index i = 3 and shift s[3] 11 times to the next character for a total cost of 11.
//
// Constraints:
//
// 1 <= s.length == t.length <= 10^5
// s and t consist only of lowercase English letters.
// nextCost.length == previousCost.length == 26
// 0 <= nextCost[i], previousCost[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn shift_distance(s: String, t: String, next_cost: Vec<i32>, prev_cost: Vec<i32>) -> i64 {
        let mut res = 0;

        for (c1, c2) in s.chars().zip(t.chars()) {
            let p1 = c1 as usize - 'a' as usize;
            let p2 = c2 as usize - 'a' as usize;

            if p1 == p2 {
                continue;
            }

            let mut cost_next = 0;
            for j in 0..((p2 + 26 - p1) % 26) {
                cost_next += next_cost[(p1 + j) % 26] as i64;
            }

            let mut cost_prev = 0;
            for j in 0..((p1 + 26 - p2) % 26) {
                cost_prev += prev_cost[(p1 + 26 - j) % 26] as i64;
            }

            res += cost_next.min(cost_prev);
        }

        res
    }
}

#[test]
fn test() {
    let s = "abab".to_string();
    let t = "baba".to_string();
    let next_cost = vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let prev_cost = vec![1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(Solution::shift_distance(s, t, next_cost, prev_cost), 2);

    let s = "leet".to_string();
    let t = "code".to_string();
    let next_cost = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let prev_cost = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(Solution::shift_distance(s, t, next_cost, prev_cost), 31);
}
