#![allow(dead_code)]

// 593. Valid Square
// https://leetcode.com/problems/valid-square/
//
// Given the coordinates of four points in 2D space p1, p2, p3 and p4, return true if the four points construct a square.
//
// The coordinate of a point pi is represented as [xi, yi]. The input is not given in any order.
//
// A valid square has four equal sides with positive length and four equal angles (90-degree angles).
//
// Example 1:
//
// Input: p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1]
// Output: true
//
// Example 2:
//
// Input: p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,12]
// Output: false
//
// Example 3:
//
// Input: p1 = [1,0], p2 = [-1,0], p3 = [0,1], p4 = [0,-1]
// Output: true
//
// Constraints:
//
// - p1.length == p2.length == p3.length == p4.length == 2
// - -10^4 <= xi, yi <= 10^4
//

struct Solution;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut points = std::collections::HashSet::new();
        points.insert(p1);
        points.insert(p2);
        points.insert(p3);
        points.insert(p4);

        if points.len() != 4 {
            return false;
        }

        let mut distances = std::collections::HashSet::new();
        for p1 in points.iter() {
            for p2 in points.iter() {
                if p1 != p2 {
                    distances.insert(Self::distance(p1, p2));
                }
            }
        }

        distances.len() == 2
    }

    fn distance(p1: &[i32], p2: &[i32]) -> i32 {
        let x = p1[0] - p2[0];
        let y = p1[1] - p2[1];
        x * x + y * y
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
        true
    );
    assert_eq!(
        Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 12]),
        false
    );
    assert_eq!(
        Solution::valid_square(vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]),
        true
    );
}
