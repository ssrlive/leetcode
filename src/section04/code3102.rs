#![allow(dead_code)]

// 3102. Minimize Manhattan Distances
// https://leetcode.com/problems/minimize-manhattan-distances/
// https://leetcode.cn/problems/minimize-manhattan-distances/
//
// Hard
//
// You are given a array points representing integer coordinates of some points on a 2D plane, where points[i] = [xi, yi].
//
// The distance between two points is defined as their Manhattan distance.
//
// Return the minimum possible value for maximum distance between any two points by removing exactly one point.
//
// Example 1:
//
// Input: points = [[3,10],[5,15],[10,2],[4,4]]
//
// Output: 12
//
// Explanation:
//
// The maximum distance after removing each point is the following:
//
// - After removing the 0th point the maximum distance is between points (5, 15) and (10, 2), which is |5 - 10| + |15 - 2| = 18.
// - After removing the 1st point the maximum distance is between points (3, 10) and (10, 2), which is |3 - 10| + |10 - 2| = 15.
// - After removing the 2nd point the maximum distance is between points (5, 15) and (4, 4), which is |5 - 4| + |15 - 4| = 12.
// - After removing the 3rd point the maximum distance is between points (5, 15) and (10, 2), which is |5 - 10| + |15 - 2| = 18.
//
// 12 is the minimum possible maximum distance between any two points after removing exactly one point.
//
// Example 2:
//
// Input: points = [[1,1],[1,1],[1,1]]
//
// Output: 0
//
// Explanation:
//
// Removing any of the points results in the maximum distance between any two points of 0.
//
// Constraints:
//
//     3 <= points.length <= 10^5
//     points[i].length == 2
//     1 <= points[i][0], points[i][1] <= 10^8
//

struct Solution;

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut sum = vec![(0, 0); n];
        let mut diff = vec![(0, 0); n];

        for i in 0..n {
            sum[i] = (points[i][0] + points[i][1], i);
            diff[i] = (points[i][0] - points[i][1], i);
        }

        sum.sort();
        diff.sort();

        let mut ans = std::i32::MAX;

        for i in 0..n {
            let mut left = 0;
            let mut right = n - 1;

            if sum[left].1 == i {
                left += 1;
            }
            if sum[right].1 == i {
                right -= 1;
            }

            let sum_val = sum[right].0 - sum[left].0;

            left = 0;
            right = n - 1;

            if diff[left].1 == i {
                left += 1;
            }
            if diff[right].1 == i {
                right -= 1;
            }

            let diff_val = diff[right].0 - diff[left].0;

            ans = ans.min(sum_val.max(diff_val));
        }

        ans
    }
}

#[test]
fn test() {
    let points = vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]];
    let res = 12;
    assert_eq!(Solution::minimum_distance(points), res);

    let points = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
    let res = 0;
    assert_eq!(Solution::minimum_distance(points), res);
}
