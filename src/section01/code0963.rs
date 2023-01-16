#![allow(dead_code)]

// 963. Minimum Area Rectangle II
// https://leetcode.com/problems/minimum-area-rectangle-ii/
// https://leetcode.cn/problems/minimum-area-rectangle-ii/
//
// You are given an array of points in the X-Y plane points where points[i] = [xi, yi].
//
// Return the minimum area of any rectangle formed from these points, with sides not necessarily parallel to the X and Y axes.
// If there is not any such rectangle, return 0.
//
// Answers within 10-5 of the actual answer will be accepted.
//
// Example 1:
//
// Input: points = [[1,2],[2,1],[1,0],[0,1]]
// Output: 2.00000
// Explanation: The minimum area rectangle occurs at [1,2],[2,1],[1,0],[0,1], with an area of 2.
//
// Example 2:
//
// Input: points = [[0,1],[2,1],[1,1],[1,0],[2,0]]
// Output: 1.00000
// Explanation: The minimum area rectangle occurs at [1,0],[1,1],[2,1],[2,0], with an area of 1.
//
// Example 3:
//
// Input: points = [[0,3],[1,2],[3,1],[1,3],[2,1]]
// Output: 0
// Explanation: There is no possible rectangle to form from these points.
//
// Constraints:
//
// - 1 <= points.length <= 50
// - points[i].length == 2
// - 0 <= xi, yi <= 4 * 10^4
// - All the given points are unique.
//

struct Solution;

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        fn d2(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
            (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
        }
        let points = points
            .iter()
            .map(|v| v.iter().map(|&x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut res = 0;
        let mut m = std::collections::HashMap::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let pt_i_0 = points[i][0];
                let pt_i_1 = points[i][1];
                let pt_j_0 = points[j][0];
                let pt_j_1 = points[j][1];
                let center = (((pt_i_0 + pt_j_0) as u64) << 16) + (pt_i_1 + pt_j_1) as u64;
                let v = vec![pt_i_0, pt_i_1, pt_j_0, pt_j_1];
                m.entry(center).or_insert_with(Vec::new).push(v);
            }
        }
        for (_center, points) in m {
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    let p1 = &points[i];
                    let p2 = &points[j];
                    if (p1[0] - p2[0]) * (p1[0] - p2[2]) + (p1[1] - p2[1]) * (p1[1] - p2[3]) == 0 {
                        let area = d2(p1[0], p1[1], p2[0], p2[1]) * d2(p1[0], p1[1], p2[2], p2[3]);
                        if res == 0 || res > area {
                            res = area;
                        }
                    }
                }
            }
        }
        (res as f64).sqrt()
    }
}

#[test]
fn test() {
    let points = vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]];
    assert_eq!(Solution::min_area_free_rect(points), 2.0);

    let points = vec![vec![0, 1], vec![2, 1], vec![1, 1], vec![1, 0], vec![2, 0]];
    assert_eq!(Solution::min_area_free_rect(points), 1.0);

    let points = vec![vec![0, 3], vec![1, 2], vec![3, 1], vec![1, 3], vec![2, 1]];
    assert_eq!(Solution::min_area_free_rect(points), 0.0);
}
