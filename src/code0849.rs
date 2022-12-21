#![allow(dead_code)]

// 849. Maximize Distance to Closest Person
// https://leetcode.com/problems/maximize-distance-to-closest-person/
// https://leetcode.cn/problems/maximize-distance-to-closest-person/
//
// You are given an array representing a row of seats where seats[i] = 1 represents a person sitting in the ith seat,
// and seats[i] = 0 represents that the ith seat is empty (0-indexed).
//
// There is at least one empty seat, and at least one person sitting.
//
// Alex wants to sit in the seat such that the distance between him and the closest person to him is maximized.
//
// Return that maximum distance to the closest person.
//
// Example 1:
//
// Input: seats = [1,0,0,0,1,0,1]
// Output: 2
// Explanation:
// If Alex sits in the second open seat (i.e. seats[2]), then the closest person has distance 2.
// If Alex sits in any other open seat, the closest person has distance 1.
// Thus, the maximum distance to the closest person is 2.
//
// Example 2:
//
// Input: seats = [1,0,0,0]
// Output: 3
// Explanation:
// If Alex sits in the last seat (i.e. seats[3]), the closest person is 3 seats away.
// This is the maximum distance possible, so the answer is 3.
//
// Example 3:
//
// Input: seats = [0,1]
// Output: 1
//
// Constraints:
//
// - 2 <= seats.length <= 2 * 10^4
// - seats[i] is 0 or 1.
// - At least one seat is empty.
// - At least one seat is occupied.
//

struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_dist = 0;
        let mut prev = -1;
        for (i, &seat) in seats.iter().enumerate() {
            if seat == 1 {
                if prev == -1 {
                    max_dist = i as i32;
                } else {
                    max_dist = std::cmp::max(max_dist, (i - prev as usize) as i32 / 2);
                }
                prev = i as i32;
            }
        }
        max_dist = std::cmp::max(max_dist, (seats.len() - 1 - prev as usize) as i32);
        max_dist
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
    assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0]), 3);
    assert_eq!(Solution::max_dist_to_closest(vec![0, 1]), 1);
}
