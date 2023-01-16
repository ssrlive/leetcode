#![allow(dead_code)]

// 1014. Best Sightseeing Pair
// https://leetcode.com/problems/best-sightseeing-pair/
// https://leetcode.cn/problems/best-sightseeing-pair/
//
// You are given an integer array values where values[i] represents the value of the ith sightseeing spot. Two sightseeing spots i and j have a distance j - i between them.
//
// The score of a pair (i < j) of sightseeing spots is values[i] + values[j] + i - j: the sum of the values of the sightseeing spots, minus the distance between them.
//
// Return the maximum score of a pair of sightseeing spots.
//
// Example 1:
//
// Input: values = [8,1,5,2,6]
// Output: 11
// Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11
//
// Example 2:
//
// Input: values = [1,2]
// Output: 2
//
// Constraints:
//
// - 2 <= values.length <= 5 * 10^4
// - 1 <= values[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        values
            .iter()
            .enumerate()
            .rev()
            .fold((-10_000_000, -10_000_000), |(max, acc), (index, val)| {
                let index = index as i32;
                (max.max(val + index + acc), acc.max(val - index))
            })
            .0
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![8, 1, 5, 2, 6], 11),
        (vec![1, 2], 2),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 18),
    ];
    for (values, expected) in cases {
        assert_eq!(Solution::max_score_sightseeing_pair(values), expected);
    }
}
