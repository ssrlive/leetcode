#![allow(dead_code)]

// 391. Perfect Rectangle
// https://leetcode.com/problems/perfect-rectangle/
//
// Given an array rectangles where rectangles[i] = [xi, yi, ai, bi] represents an axis-aligned rectangle.
// The bottom-left point of the rectangle is (xi, yi) and the top-right point of it is (ai, bi).
//
// Return true if all the rectangles together form an exact cover of a rectangular region.
//
// Example 1:
//
// Input: rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
// Output: true
// Explanation: All 5 rectangles together form an exact cover of a rectangular region.
//
// Example 2:
//
// Input: rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
// Output: false
// Explanation: Because there is a gap between the two rectangular regions.
//
// Example 3:
//
// Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
// Output: false
// Explanation: Because two of the rectangles overlap with each other.
//
// Constraints:
//
// - 1 <= rectangles.length <= 2 * 104
// - rectangles[i].length == 4
// - -105 <= xi, yi, ai, bi <= 105
//

struct Solution;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut area = 0;
        let mut x1 = i32::MAX;
        let mut y1 = i32::MAX;
        let mut x2 = i32::MIN;
        let mut y2 = i32::MIN;
        let mut set = std::collections::HashSet::new();
        for r in rectangles {
            x1 = x1.min(r[0]);
            y1 = y1.min(r[1]);
            x2 = x2.max(r[2]);
            y2 = y2.max(r[3]);
            area += (r[2] - r[0]) * (r[3] - r[1]);
            let p1 = (r[0], r[1]);
            let p2 = (r[0], r[3]);
            let p3 = (r[2], r[1]);
            let p4 = (r[2], r[3]);
            if !set.insert(p1) {
                set.remove(&p1);
            }
            if !set.insert(p2) {
                set.remove(&p2);
            }
            if !set.insert(p3) {
                set.remove(&p3);
            }
            if !set.insert(p4) {
                set.remove(&p4);
            }
        }
        area == (x2 - x1) * (y2 - y1)
            && set.len() == 4
            && set.contains(&(x1, y1))
            && set.contains(&(x1, y2))
            && set.contains(&(x2, y1))
            && set.contains(&(x2, y2))
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4]
        ]),
        true
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 2, 3],
            vec![1, 3, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![2, 2, 4, 4]
        ]),
        false
    );
}
