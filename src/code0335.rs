#![allow(dead_code)]

// 335. Self Crossing
// https://leetcode.com/problems/self-crossing/
//
// You are given an array of integers distance.
//
// You start at point (0,0) on an X-Y plane and you move distance[0] meters to the north, then distance[1] meters to the west,
// distance[2] meters to the south, distance[3] meters to the east, and so on. In other words, after each move,
// your direction changes counter-clockwise.
//
// Return true if your path crosses itself, and false if it does not.
//
// Example 1:
//
// ┌───┐
// │   │
// └───┼──>
//     │
//
// Input: [2,1,1,2]
// Output: true
//
// Example 2:
//
// ┌──────┐
// │      │
// │
// │
// └────────────>
//
// Input: [1,2,3,4]
// Output: false
//
// Example 3:
//
// ┌───┐
// │   │
// └───┼>
//
// Input: [1,1,1,1]
// Output: true
//
// Constraints:
//
// - 1 <= distance.length <= 105
// - 1 <= distance[i] <= 105
//

struct Solution;

impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        let mut e = 0;
        let mut f = 0;
        for a in distance.iter() {
            if (d >= b && b > 0) && (*a >= c || ((*a >= c - e) && c - e >= 0) && (f >= d - b)) {
                return true;
            }
            f = e;
            e = d;
            d = c;
            c = b;
            b = *a;
        }
        false
    }
}

#[test]
fn test_is_self_crossing() {
    assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 2]), true);
    assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4]), false);
    assert_eq!(Solution::is_self_crossing(vec![1, 1, 1, 1]), true);
    assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 1, 1]), true);
    assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 2, 1, 1]), true);
    assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 2, 1, 1, 1]), true);
    assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 2, 1, 1, 1, 1]), true);
    assert_eq!(Solution::is_self_crossing(vec![3, 3, 3, 2, 1, 1]), false);
}
