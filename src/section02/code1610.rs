#![allow(dead_code)]

/*

// 1610. Maximum Number of Visible Points
// https://leetcode.com/problems/maximum-number-of-visible-points/
// https://leetcode.cn/problems/maximum-number-of-visible-points/
//
// Hard
//
// You are given an array points, an integer angle, and your location, where location = [posx, posy] and points[i] = [xi, yi] both denote integral coordinates on the X-Y plane.

Initially, you are facing directly east from your position. You cannot move from your position, but you can rotate. In other words, posx and posy cannot be changed. Your field of view in degrees is represented by angle, determining how wide you can see from any given view direction. Let d be the amount in degrees that you rotate counterclockwise. Then, your field of view is the inclusive range of angles [d - angle/2, d + angle/2].

Your browser does not support the video tag or this video format.

You can see some set of points if, for each point, the angle formed by the point, your position, and the immediate east direction from your position is in your field of view.

There can be multiple points at one coordinate. There may be points at your location, and you can always see these points regardless of your rotation. Points do not obstruct your vision to other points.

Return the maximum number of points you can see.

Example 1:

Input: points = [[2,1],[2,2],[3,3]], angle = 90, location = [1,1]
Output: 3
Explanation: The shaded region represents your field of view. All points can be made visible in your field of view, including [3,3] even though [2,2] is in front and in the same line of sight.

Example 2:

Input: points = [[2,1],[2,2],[3,4],[1,1]], angle = 90, location = [1,1]
Output: 4
Explanation: All points can be made visible in your field of view, including the one at your location.

Example 3:

Input: points = [[1,0],[2,1]], angle = 13, location = [1,1]
Output: 1
Explanation: You can only see one of the two points, as shown above.

Constraints:

    1 <= points.length <= 10^5
    points[i].length == 2
    location.length == 2
    0 <= angle < 360
    0 <= posx, posy, xi, yi <= 100
*/

struct Solution;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        use std::f64::consts::PI;
        if points.is_empty() {
            return 0;
        }
        let mut extra = 0;
        let mut angles = points
            .iter()
            .filter(|p| {
                if p[0] == location[0] && p[1] == location[1] {
                    extra += 1;
                    false
                } else {
                    true
                }
            })
            .map(|p| ((location[1] - p[1]) as f64).atan2((location[0] - p[0]) as f64))
            .map(|a| 360.0 * (if a < 0.0 { a + 2.0 * PI } else { a }) / (2.0 * PI))
            .collect::<Vec<_>>();
        for i in 0..angles.len() {
            angles.push(angles[i] + 360.0);
        }
        if angles.is_empty() {
            return extra as i32;
        }

        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let angle = angle as f64;
        let mut best = 1;
        let mut l = 0;
        let mut r = 1;
        while r < angles.len() && l < angles.len() {
            while r < angles.len() && angles[r] - angles[l] <= angle {
                r += 1;
            }
            best = best.max(r - l);
            l += 1;
        }
        (extra + best) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![2, 1], vec![2, 2], vec![3, 3]], 90, vec![1, 1], 3),
        (vec![vec![2, 1], vec![2, 2], vec![3, 4], vec![1, 1]], 90, vec![1, 1], 4),
        (vec![vec![1, 0], vec![2, 1]], 13, vec![1, 1], 1),
    ];
    for (points, angle, location, expected) in cases {
        assert_eq!(Solution::visible_points(points, angle, location), expected);
    }
}
