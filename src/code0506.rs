#![allow(dead_code)]

// 506. Relative Ranks
// https://leetcode.com/problems/relative-ranks/
//
// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.
//
// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:
//
// The 1st place athlete's rank is "Gold Medal".
// The 2nd place athlete's rank is "Silver Medal".
// The 3rd place athlete's rank is "Bronze Medal".
// For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
// Return an array answer of size n where answer[i] is the rank of the ith athlete.
//
// Example 1:
//
// Input: score = [5,4,3,2,1]
// Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
// Explanation: The placements are [1st, 2nd, 3rd, 4th, 5th].
//
// Example 2:
//
// Input: score = [10,3,8,9,4]
// Output: ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
// Explanation: The placements are [1st, 5th, 3rd, 2nd, 4th].
//
// Constraints:
//
// - n == score.length
// - 1 <= n <= 104
// - 0 <= score[i] <= 106
// All the values in score are unique.
//

struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let l = score.len();

        let mut score_sort = score.clone();
        score_sort.sort_unstable();

        let m1 = score_sort.pop().unwrap_or(-1);
        let m2 = score_sort.pop().unwrap_or(-1);
        let m3 = score_sort.pop().unwrap_or(-1);

        score
            .iter()
            .map(|&x| match x {
                _ if x == m1 => "Gold Medal".to_string(),
                _ if x == m2 => "Silver Medal".to_string(),
                _ if x == m3 => "Bronze Medal".to_string(),
                _ => score_sort.binary_search(&x).map_or(0, |i| l - i).to_string(),
            })
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    );
    assert_eq!(
        Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
        vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
    );
}
