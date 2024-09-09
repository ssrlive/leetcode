#![allow(dead_code)]

// 3281. Maximize Score of Numbers in Ranges
// https://leetcode.com/problems/maximize-score-of-numbers-in-ranges/
// https://leetcode.cn/problems/maximize-score-of-numbers-in-ranges/
//
// Medium
//
// You are given an array of integers start and an integer d, representing n intervals [start[i], start[i] + d].
//
// You are asked to choose n integers where the ith integer must belong to the ith interval.
// The score of the chosen integers is defined as the minimum absolute difference between any two integers that have been chosen.
//
// Return the maximum possible score of the chosen integers.
//
// Example 1:
//
// Input: start = [6,0,3], d = 2
//
// Output: 4
//
// Explanation:
//
// The maximum possible score can be obtained by choosing integers: 8, 0, and 4.
// The score of these chosen integers is min(|8 - 0|, |8 - 4|, |0 - 4|) which equals 4.
//
// Example 2:
//
// Input: start = [2,6,13,13], d = 5
//
// Output: 5
//
// Explanation:
//
// The maximum possible score can be obtained by choosing integers: 2, 7, 13, and 18.
// The score of these chosen integers is min(|2 - 7|, |2 - 13|, |2 - 18|, |7 - 13|, |7 - 18|, |13 - 18|) which equals 5.
//
// Constraints:
//
//     2 <= start.length <= 10^5
//     0 <= start[i] <= 10^9
//     0 <= d <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
        fn check(start: &[i64], d: i64, mid: i64) -> bool {
            let mut prev = start[0];
            for &start_i in start.iter().skip(1) {
                let next = std::cmp::max(prev + mid, start_i);
                if next > start_i + d {
                    return false;
                }
                prev = next;
            }
            true
        }

        let mut start = start.iter().map(|&x| x as i64).collect::<Vec<_>>();
        start.sort_unstable();
        let mut low = 0;
        let mut hi = start[start.len() - 1] - start[0] + d as i64;

        let mut ans = 0;
        while low <= hi {
            let mid = (low + hi) / 2;
            if check(&start, d as _, mid) {
                ans = mid;
                low = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_possible_score(vec![6, 0, 3], 2), 4);
    assert_eq!(Solution::max_possible_score(vec![2, 6, 13, 13], 5), 5);
    assert_eq!(Solution::max_possible_score(vec![1000000000, 0], 1000000000), 2000000000);
}
