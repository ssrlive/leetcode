#![allow(dead_code)]

// 939. Minimum Area Rectangle
// https://leetcode.com/problems/minimum-area-rectangle/
// https://leetcode.cn/problems/minimum-area-rectangle/
//
// You are given an array of points in the X-Y plane points where points[i] = [xi, yi].
//
// Return the minimum area of a rectangle formed from these points, with sides parallel to the X and Y axes.
// If there is not any such rectangle, return 0.
//
// Example 1:
//
// Input: points = [[1,1],[1,3],[3,1],[3,3],[2,2]]
// Output: 4
//
// Example 2:
//
// Input: points = [[1,1],[1,3],[3,1],[3,3],[4,1],[4,3]]
// Output: 2
//
// Constraints:
//
// - 1 <= points.length <= 500
// - points[i].length == 2
// - 0 <= xi, yi <= 4 * 10^4
// - All the given points are unique.
//

struct Solution;

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for p in points.iter() {
            map.entry(p[0]).or_insert_with(std::collections::HashSet::new);
            map.get_mut(&p[0]).unwrap().insert(p[1]);
        }
        let mut min = std::i32::MAX;
        for p1 in points.iter() {
            for p2 in points.iter() {
                if p1[0] == p2[0] || p1[1] == p2[1] {
                    continue;
                }
                if map.get(&p1[0]).unwrap().contains(&p2[1]) && map.get(&p2[0]).unwrap().contains(&p1[1]) {
                    min = min.min((p1[0] - p2[0]).abs() * (p1[1] - p2[1]).abs());
                }
            }
        }
        if min == std::i32::MAX {
            0
        } else {
            min
        }
    }
}

#[test]
fn test() {
    let points = vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![2, 2]];
    assert_eq!(Solution::min_area_rect(points), 4);
    let points = vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![4, 1], vec![4, 3]];
    assert_eq!(Solution::min_area_rect(points), 2);
}
