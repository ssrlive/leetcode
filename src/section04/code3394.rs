#![allow(dead_code)]

// 3394. Check if Grid can be Cut into Sections
// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/
// https://leetcode.cn/problems/check-if-grid-can-be-cut-into-sections/
//
// Medium
//
// You are given an integer n representing the dimensions of an n x n grid, with the origin at the
// bottom-left corner of the grid. You are also given a 2D array of coordinates rectangles,
// where rectangles[i] is in the form [startx, starty, endx, endy], representing a rectangle on the grid.
// Each rectangle is defined as follows:
//
// (startx, starty): The bottom-left corner of the rectangle.
// (endx, endy): The top-right corner of the rectangle.
// Note that the rectangles do not overlap. Your task is to determine if it is possible to make either
// two horizontal or two vertical cuts on the grid such that:
//
// Each of the three resulting sections formed by the cuts contains at least one rectangle.
// Every rectangle belongs to exactly one section.
// Return true if such cuts can be made; otherwise, return false.
//
// Example 1:
//
// Input: n = 5, rectangles = [[1,0,5,2],[0,2,2,4],[3,2,5,3],[0,4,4,5]]
//
// Output: true
//
// Explanation:
//
// The grid is shown in the diagram. We can make horizontal cuts at y = 2 and y = 4. Hence, output is true.
//
// Example 2:
//
// Input: n = 4, rectangles = [[0,0,1,1],[2,0,3,4],[0,2,2,3],[3,0,4,3]]
//
// Output: true
//
// Explanation:
//
// We can make vertical cuts at x = 2 and x = 3. Hence, output is true.
//
// Example 3:
//
// Input: n = 4, rectangles = [[0,2,2,4],[1,0,3,2],[2,2,3,4],[3,0,4,2],[3,2,4,4]]
//
// Output: false
//
// Explanation:
//
// We cannot make two horizontal or two vertical cuts that satisfy the conditions. Hence, output is false.
//
// Constraints:
//
// 3 <= n <= 10^9
// 3 <= rectangles.length <= 10^5
// 0 <= rectangles[i][0] < rectangles[i][2] <= n
// 0 <= rectangles[i][1] < rectangles[i][3] <= n
// No two rectangles overlap.
//

struct Solution;

impl Solution {
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        fn merge_intervals(intervals: &mut [Vec<i32>]) -> Vec<Vec<i32>> {
            if intervals.len() <= 1 {
                return intervals.to_vec();
            }
            intervals.sort();
            let mut merged = Vec::new();
            merged.push(intervals[0].clone());
            for intervals_i in intervals.iter().skip(1) {
                if intervals_i[0] < merged.last().unwrap()[1] {
                    merged.last_mut().unwrap()[1] = merged.last().unwrap()[1].max(intervals_i[1]);
                } else {
                    merged.push(intervals_i.clone());
                }
            }
            merged
        }

        let mut vertical = Vec::new();
        let mut horizontal = Vec::new();
        for it in rectangles.iter() {
            vertical.push(vec![it[1], it[3]]);
            horizontal.push(vec![it[0], it[2]]);
        }
        let merge_h = merge_intervals(&mut horizontal);
        let merge_v = merge_intervals(&mut vertical);
        merge_h.len() >= 3 || merge_v.len() >= 3
    }
}

#[test]
fn test() {
    let n = 5;
    let rectangles = vec![vec![1, 0, 5, 2], vec![0, 2, 2, 4], vec![3, 2, 5, 3], vec![0, 4, 4, 5]];
    let res = true;
    assert_eq!(Solution::check_valid_cuts(n, rectangles), res);

    let n = 4;
    let rectangles = vec![vec![0, 0, 1, 1], vec![2, 0, 3, 4], vec![0, 2, 2, 3], vec![3, 0, 4, 3]];
    let res = true;
    assert_eq!(Solution::check_valid_cuts(n, rectangles), res);

    let n = 4;
    let rectangles = vec![
        vec![0, 2, 2, 4],
        vec![1, 0, 3, 2],
        vec![2, 2, 3, 4],
        vec![3, 0, 4, 2],
        vec![3, 2, 4, 4],
    ];
    let res = false;
    assert_eq!(Solution::check_valid_cuts(n, rectangles), res);
}
