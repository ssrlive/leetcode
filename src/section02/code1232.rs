#![allow(dead_code)]

// 1232. Check If It Is a Straight Line
// https://leetcode.com/problems/check-if-it-is-a-straight-line/
// https://leetcode.cn/problems/check-if-it-is-a-straight-line/
//
// Easy
//
// You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.
//
// Example 1:
//
// Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
// Output: true
//
// Example 2:
//
// Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
// Output: false
//
// Constraints:
//
// -    2 <= coordinates.length <= 1000
// -    coordinates[i].length == 2
// -    -10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
// -    coordinates contains no duplicate point.
//

struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (x0, y0) = (coordinates[0][0], coordinates[0][1]);
        let (x1, y1) = (coordinates[1][0], coordinates[1][1]);
        let (dx, dy) = (x1 - x0, y1 - y0);
        for coordinate in coordinates.iter().skip(2) {
            let (x, y) = (coordinate[0], coordinate[1]);
            if dx * (y - y0) != dy * (x - x0) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]], true),
        (vec![vec![1, 1], vec![2, 2], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]], false),
    ];
    for (coordinates, expected) in cases {
        assert_eq!(Solution::check_straight_line(coordinates), expected);
    }
}
