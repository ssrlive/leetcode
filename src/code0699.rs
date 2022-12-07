#![allow(dead_code)]

// 699. Falling Squares
// https://leetcode.com/problems/falling-squares/
//
// There are several squares being dropped onto the X-axis of a 2D plane.
//
// You are given a 2D integer array positions where positions[i] = [lefti, sideLengthi] represents the ith square with a side length of sideLengthi that is dropped with its left edge aligned with X-coordinate lefti.
//
// Each square is dropped one at a time from a height above any landed squares. It then falls downward (negative Y direction) until it either lands on the top side of another square or on the X-axis. A square brushing the left/right side of another square does not count as landing on it. Once it lands, it freezes in place and cannot be moved.
//
// After each square is dropped, you must record the height of the current tallest stack of squares.
//
// Return an integer array ans where ans[i] represents the height described above after dropping the ith square.
//
// Example 1:
//
// Input: positions = [[1,2],[2,3],[6,1]]
// Output: [2,5,5]
// Explanation:
// After the first drop, the tallest stack is square 1 with a height of 2.
// After the second drop, the tallest stack is squares 1 and 2 with a height of 5.
// After the third drop, the tallest stack is still squares 1 and 2 with a height of 5.
// Thus, we return an answer of [2, 5, 5].
//
// Example 2:
//
// Input: positions = [[100,100],[200,100]]
// Output: [100,100]
// Explanation:
// After the first drop, the tallest stack is square 1 with a height of 100.
// After the second drop, the tallest stack is either square 1 or square 2, both with heights of 100.
// Thus, we return an answer of [100, 100].
// Note that square 2 only brushes the right side of square 1, which does not count as landing on it.
//
// Constraints:
//
// - 1 <= positions.length <= 1000
// - 1 <= lefti <= 10^8
// - 1 <= sideLengthi <= 10^6
//

struct Solution;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut heights = vec![];
        let mut max_height = 0;
        for pos in positions {
            let left = pos[0];
            let right = left + pos[1];
            let mut height = pos[1];
            for (l, r, h) in heights.iter() {
                if left < *r && right > *l {
                    height = height.max(*h + pos[1]);
                }
            }
            heights.push((left, right, height));
            max_height = max_height.max(height);
            ans.push(max_height);
        }
        ans
    }
}

#[test]
fn test() {
    let positions = vec![vec![1, 2], vec![2, 3], vec![6, 1]];
    let res = vec![2, 5, 5];
    assert_eq!(Solution::falling_squares(positions), res);

    let positions = vec![vec![100, 100], vec![200, 100]];
    let res = vec![100, 100];
    assert_eq!(Solution::falling_squares(positions), res);
}
