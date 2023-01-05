#![allow(dead_code)]

// 973. K Closest Points to Origin
// https://leetcode.com/problems/k-closest-points-to-origin/
// https://leetcode.cn/problems/k-closest-points-to-origin/
//
// Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane and an integer k,
// return the k closest points to the origin (0, 0).
//
// The distance between two points on the X-Y plane is the Euclidean distance (i.e., √(x1 - x2)2 + (y1 - y2)2).
//
// You may return the answer in any order. The answer is guaranteed to be unique (except for the order that it is in).
//
// Example 1:
//
// Input: points = [[1,3],[-2,2]], k = 1
// Output: [[-2,2]]
// Explanation:
// The distance between (1, 3) and the origin is sqrt(10).
// The distance between (-2, 2) and the origin is sqrt(8).
// Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
// We only want the closest k = 1 points from the origin, so the answer is just [[-2,2]].
//
// Example 2:
//
// Input: points = [[3,3],[5,-1],[-2,4]], k = 2
// Output: [[3,3],[-2,4]]
// Explanation: The answer [[-2,4],[3,3]] would also be accepted.
//
// Constraints:
//
// - 1 <= k <= points.length <= 10^4
// - -10^4 < xi, yi < 10^4
//

struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut minheap: BinaryHeap<Reverse<(i32, Vec<i32>)>> = BinaryHeap::new();

        for coord in points {
            let dist = coord[0].pow(2) + coord[1].pow(2);
            minheap.push(Reverse((dist, coord)));
        }

        let mut res: Vec<Vec<i32>> = Vec::new();

        while let Some(Reverse((_, coord))) = minheap.pop() {
            res.push(coord);
            if res.len() == k as usize {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let points = vec![vec![1, 3], vec![-2, 2]];
    let k = 1;
    let res = vec![vec![-2, 2]];
    assert_eq!(Solution::k_closest(points, k), res);

    let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
    let k = 2;
    let res = vec![vec![3, 3], vec![-2, 4]];
    assert_eq!(Solution::k_closest(points, k), res);
}
