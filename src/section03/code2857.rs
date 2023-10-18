#![allow(dead_code)]

// 2857. Count Pairs of Points With Distance k
// https://leetcode.com/problems/count-pairs-of-points-with-distance-k/
// https://leetcode.cn/problems/count-pairs-of-points-with-distance-k/
//
// Medium
//
// You are given a 2D integer array coordinates and an integer k,
// where coordinates[i] = [xi, yi] are the coordinates of the ith point in a 2D plane.
//
// We define the distance between two points (x1, y1) and (x2, y2) as (x1 XOR x2) + (y1 XOR y2)
// where XOR is the bitwise XOR operation.
//
// Return the number of pairs (i, j) such that i < j and the distance between points i and j is equal to k.
//
// Example 1:
//
// Input: coordinates = [[1,2],[4,2],[1,3],[5,2]], k = 5
// Output: 2
// Explanation: We can choose the following pairs:
// - (0,1): Because we have (1 XOR 4) + (2 XOR 2) = 5.
// - (2,3): Because we have (1 XOR 5) + (3 XOR 2) = 5.
//
// Example 2:
//
// Input: coordinates = [[1,3],[1,3],[1,3],[1,3],[1,3]], k = 0
// Output: 10
// Explanation: Any two chosen pairs will have a distance of 0. There are 10 ways to choose two pairs.
//
// Constraints:
//
// 2 <= coordinates.length <= 50000
// 0 <= xi, yi <= 10^6
// 0 <= k <= 100
//

struct Solution;

impl Solution {
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut hm = std::collections::HashMap::new();
        let mut ret = 0;

        for c in &coordinates {
            for kx in 0..=k {
                ret += *hm.get(&(c[0] ^ kx, c[1] ^ (k - kx))).unwrap_or(&0);
            }
            *hm.entry((c[0], c[1])).or_insert(0) += 1;
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_pairs(vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]], 5), 2);

    let v = vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3]];
    assert_eq!(Solution::count_pairs(v, 0), 10);
}
