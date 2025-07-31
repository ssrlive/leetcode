#![allow(dead_code)]

// 3625. Count Number of Trapezoids II
// https://leetcode.com/problems/count-number-of-trapezoids-ii/
// https://leetcode.cn/problems/count-number-of-trapezoids-ii/
//
// Hard
//
// You are given a 2D integer array points where points[i] = [xi, yi] represents the coordinates of the ith point on the Cartesian plane.
//
// Return the number of unique trapezoids that can be formed by choosing any four distinct points from points.
//
// A trapezoid is a convex quadrilateral with at least one pair of parallel sides. Two lines are parallel if and only if they have the same slope.
//
// Example 1:
//
// Input: points = [[-3,2],[3,0],[2,3],[3,2],[2,-3]]
//
// Output: 2
//
// Explanation:
//
// There are two distinct ways to pick four points that form a trapezoid:
//
// The points [-3,2], [2,3], [3,2], [2,-3] form one trapezoid.
// The points [2,3], [3,2], [3,0], [2,-3] form another trapezoid.
//
// Example 2:
//
// Input: points = [[0,0],[1,0],[0,1],[2,1]]
//
// Output: 1
//
// Explanation:
//
// There is only one trapezoid which can be formed.
//
// Constraints:
//
// 4 <= points.length <= 500
// –1000 <= xi, yi <= 1000
// All points are pairwise distinct.
//

struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        use std::collections::HashMap;
        let points = points.into_iter().map(|x| (x[0], x[1])).collect::<Vec<_>>();
        let mut slopes = HashMap::new();
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            for &(x2, y2) in &points[i + 1..] {
                let dx = x2 - x1;
                let dy = y2 - y1;
                let l = dx * dx + dy * dy;
                assert!(l != 0);
                let g = gcd(dx.abs(), dy.abs());
                let dx = dx / g;
                let dy = dy / g;
                let c = if dx == 0 { (x1, 1) } else { (y1 * dx - dy * x1, dx) };
                let g = gcd(c.0.abs(), c.1.abs());
                let cx = c.0 / g;
                let cy = c.1 / g;
                let (dx, dy) = (dx, dy).max((-dx, -dy));
                let (cx, cy) = (cx, cy).max((-cx, -cy));
                *slopes
                    .entry((dx, dy))
                    .or_insert_with(HashMap::new)
                    .entry((cx, cy))
                    .or_insert_with(HashMap::new)
                    .entry(l)
                    .or_insert(0) += 1;
            }
        }
        let mut cnt = 0;
        let mut par = 0;
        for p in slopes.into_values() {
            let mut seen = 0;
            let mut lls = HashMap::new();
            for ll in p.into_values() {
                let mut tba = 0;
                for (l, v) in ll {
                    cnt += seen * v;
                    tba += v;
                    let ll = *lls.entry(l).or_insert(0);
                    par += ll * v;
                    lls.insert(l, ll + v);
                }
                seen += tba;
            }
        }
        cnt - (par >> 1)
    }
}

#[test]
fn test() {
    let points = vec![vec![-3, 2], vec![3, 0], vec![2, 3], vec![3, 2], vec![2, -3]];
    assert_eq!(Solution::count_trapezoids(points), 2);

    let points = vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]];
    assert_eq!(Solution::count_trapezoids(points), 1);
}
