#![allow(dead_code)]

// 3588. Find Maximum Area of a Triangle
// https://leetcode.com/problems/find-maximum-area-of-a-triangle/
// https://leetcode.cn/problems/find-maximum-area-of-a-triangle/
//
// Medium
//
// You are given a 2D array coords of size n x 2, representing the coordinates of n points in an infinite Cartesian plane.
//
// Find twice the maximum area of a triangle with its corners at any three elements from coords,
// such that at least one side of this triangle is parallel to the x-axis or y-axis.
// Formally, if the maximum area of such a triangle is A, return 2 * A.
//
// If no such triangle exists, return -1.
//
// Note that a triangle cannot have zero area.
//
// Example 1:
//
// Input: coords = [[1,1],[1,2],[3,2],[3,3]]
//
// Output: 2
//
// Explanation:
//
// The triangle shown in the image has a base 1 and height 2. Hence its area is 1/2 * base * height = 1.
//
// Example 2:
//
// Input: coords = [[1,1],[2,2],[3,3]]
//
// Output: -1
//
// Explanation:
//
// The only possible triangle has corners (1, 1), (2, 2), and (3, 3).
// None of its sides are parallel to the x-axis or the y-axis.
//
// Constraints:
//
//     1 <= n == coords.length <= 10^5
//     1 <= coords[i][0], coords[i][1] <= 10^6
//     All coords[i] are unique.
//

struct Solution;

impl Solution {
    pub fn max_area(coords: Vec<Vec<i32>>) -> i64 {
        let coords: Vec<(i64, i64)> = coords.into_iter().map(|v| (v[0] as i64, v[1] as i64)).collect();
        let mut mpx: std::collections::HashMap<i64, (i64, i64)> = std::collections::HashMap::new();
        let mut mpy: std::collections::HashMap<i64, (i64, i64)> = std::collections::HashMap::new();
        let mut ymax = i64::MIN;
        let mut ymin = i64::MAX;
        let mut xmax = i64::MIN;
        let mut xmin = i64::MAX;
        for &(x, y) in &coords {
            mpx.entry(x).or_insert((i64::MAX, i64::MIN));
            let (min_y, max_y) = mpx.get_mut(&x).unwrap();
            *min_y = (*min_y).min(y);
            *max_y = (*max_y).max(y);

            mpy.entry(y).or_insert((i64::MAX, i64::MIN));
            let (min_x, max_x) = mpy.get_mut(&y).unwrap();
            *min_x = (*min_x).min(x);
            *max_x = (*max_x).max(x);

            xmax = xmax.max(x);
            xmin = xmin.min(x);
            ymax = ymax.max(y);
            ymin = ymin.min(y);
        }
        let mut res = 0;
        for (&x, &(min_y, max_y)) in &mpx {
            let base = (max_y - min_y).abs();
            let height = (xmax - x).max(x - xmin).abs();
            res = res.max(base * height);
        }
        for (&y, &(min_x, max_x)) in &mpy {
            let base = (max_x - min_x).abs();
            let height = (y - ymax).abs().max((y - ymin).abs());
            res = res.max(base * height);
        }
        if res == 0 { -1 } else { res }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_area(vec![vec![1, 1], vec![1, 2], vec![3, 2], vec![3, 3]]), 2);
    assert_eq!(Solution::max_area(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), -1);
}
