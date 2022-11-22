#![allow(dead_code)]

// 354. Russian Doll Envelopes
// https://leetcode.com/problems/russian-doll-envelopes/
//
// You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
//
// One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.
//
// Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).
//
// Note: You cannot rotate an envelope.
//
// Example 1:
//
// Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
// Output: 3
// Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
//
// Example 2:
//
// Input: envelopes = [[1,1],[1,1],[1,1]]
// Output: 1
//
// Constraints:
// - 1 <= envelopes.length <= 5000
// - envelopes[i].length == 2
// - 1 <= wi, hi <= 104
//
// Follow up: Could you solve this problem with time complexity O(n log(n))?

struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_by(|a, b| if a[0] == b[0] { b[1].cmp(&a[1]) } else { a[0].cmp(&b[0]) });
        let mut dp = vec![0; envelopes.len()];
        let mut len = 0;
        for e in envelopes {
            let mut i = 0;
            let mut j = len;
            while i < j {
                let m = (i + j) / 2;
                if dp[m] < e[1] {
                    i = m + 1;
                } else {
                    j = m;
                }
            }
            dp[i] = e[1];
            if i == len {
                len += 1;
            }
        }
        len as i32
    }
}

#[test]
fn test_max_envelopes() {
    assert_eq!(
        Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
        3
    );
    assert_eq!(Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]), 1);
}
