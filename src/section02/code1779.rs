#![allow(dead_code)]

/*

// 1779. Find Nearest Point That Has the Same X or Y Coordinate
Easy
693
145
Companies

You are given two integers, x and y, which represent your current location on a Cartesian grid: (x, y). You are also given an array points where each points[i] = [ai, bi] represents that a point exists at (ai, bi). A point is valid if it shares the same x-coordinate or the same y-coordinate as your location.

Return the index (0-indexed) of the valid point with the smallest Manhattan distance from your current location. If there are multiple, return the valid point with the smallest index. If there are no valid points, return -1.

The Manhattan distance between two points (x1, y1) and (x2, y2) is abs(x1 - x2) + abs(y1 - y2).

Example 1:

Input: x = 3, y = 4, points = [[1,2],[3,1],[2,4],[2,3],[4,4]]
Output: 2
Explanation: Of all the points, only [3,1], [2,4] and [4,4] are valid. Of the valid points, [2,4] and [4,4] have the smallest Manhattan distance from your current location, with a distance of 1. [2,4] has the smallest index, so return 2.

Example 2:

Input: x = 3, y = 4, points = [[3,4]]
Output: 0
Explanation: The answer is allowed to be on the same location as your current location.

Example 3:

Input: x = 3, y = 4, points = [[2,3]]
Output: -1
Explanation: There are no valid points.

Constraints:

    1 <= points.length <= 10^4
    points[i].length == 2
    1 <= x, y, ai, bi <= 10^4
*/

struct Solution;

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut dist_min = std::i32::MAX;
        let mut res_ind = -1;
        for (ind, pi) in points.iter().enumerate() {
            if x == pi[0] || y == pi[1] {
                let dist_cur = (x - pi[0]).abs() + (y - pi[1]).abs();
                if dist_cur < dist_min || res_ind == -1 && dist_cur == dist_min {
                    res_ind = ind as i32;
                    dist_min = dist_cur;
                    if dist_min == 0 {
                        break;
                    }
                }
            }
        }
        res_ind
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (3, 4, vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]], 2),
        (3, 4, vec![vec![3, 4]], 0),
        (3, 4, vec![vec![2, 3]], -1),
    ];
    for (x, y, points, expected) in cases {
        assert_eq!(Solution::nearest_valid_point(x, y, points), expected);
    }
}
