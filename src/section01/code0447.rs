#![allow(dead_code)]

// 447. Number of Boomerangs
// https://leetcode.com/problems/number-of-boomerangs/
// https://leetcode.cn/problems/number-of-boomerangs/
//
// You are given n points in the plane that are all distinct, where points[i] = [xi, yi].
// A boomerang is a tuple of points (i, j, k) such that the distance between i and j equals the distance
// between i and k (the order of the tuple matters).
//
// Return the number of boomerangs.
//
// Example 1:
//
// Input: points = [[0,0],[1,0],[2,0]]
// Output: 2
// Explanation: The two boomerangs are [[1,0],[0,0],[2,0]] and [[1,0],[2,0],[0,0]].
//
// Example 2:
//
// Input: points = [[1,1],[2,2],[3,3]]
// Output: 2
//
// Example 3:
//
// Input: points = [[1,1]]
// Output: 0
//
// Constraints:
//
// - n == points.length
// - 1 <= n <= 500
// - points[i].length == 2
// - -10^4 <= xi, yi <= 10^4
// - All the points are unique.
//

struct Solution;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..points.len() {
            let mut map = std::collections::HashMap::new();
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let dist = (points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2);
                *map.entry(dist).or_insert(0) += 1;
            }
            for (_, v) in map {
                ans += v * (v - 1);
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]), 2);
    assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 2);
    assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1]]), 0);
}
