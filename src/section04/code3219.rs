#![allow(dead_code)]

// 3219. Minimum Cost for Cutting Cake II
// https://leetcode.com/problems/minimum-cost-for-cutting-cake-ii/
// https://leetcode.cn/problems/minimum-cost-for-cutting-cake-ii/
//
// Hard
//
// There is an m x n cake that needs to be cut into 1 x 1 pieces.
//
// You are given integers m, n, and two arrays:
//
//     horizontalCut of size m - 1, where horizontalCut[i] represents the cost to cut along the horizontal line i.
//     verticalCut of size n - 1, where verticalCut[j] represents the cost to cut along the vertical line j.
//
// In one operation, you can choose any piece of cake that is not yet a 1 x 1 square and perform one of the following cuts:
//
//     Cut along a horizontal line i at a cost of horizontalCut[i].
//     Cut along a vertical line j at a cost of verticalCut[j].
//
// After the cut, the piece of cake is divided into two distinct pieces.
//
// The cost of a cut depends only on the initial cost of the line and does not change.
//
// Return the minimum total cost to cut the entire cake into 1 x 1 pieces.
//
// Example 1:
//
// Input: m = 3, n = 2, horizontalCut = [1,3], verticalCut = [5]
//
// Output: 13
//
// Explanation:
//
//     Perform a cut on the vertical line 0 with cost 5, current total cost is 5.
//     Perform a cut on the horizontal line 0 on 3 x 1 subgrid with cost 1.
//     Perform a cut on the horizontal line 0 on 3 x 1 subgrid with cost 1.
//     Perform a cut on the horizontal line 1 on 2 x 1 subgrid with cost 3.
//     Perform a cut on the horizontal line 1 on 2 x 1 subgrid with cost 3.
//
// The total cost is 5 + 1 + 1 + 3 + 3 = 13.
//
// Example 2:
//
// Input: m = 2, n = 2, horizontalCut = [7], verticalCut = [4]
//
// Output: 15
//
// Explanation:
//
//     Perform a cut on the horizontal line 0 with cost 7.
//     Perform a cut on the vertical line 0 on 1 x 2 subgrid with cost 4.
//     Perform a cut on the vertical line 0 on 1 x 2 subgrid with cost 4.
//
// The total cost is 7 + 4 + 4 = 15.
//
// Constraints:
//
//     1 <= m, n <= 10^5
//     horizontalCut.length == m - 1
//     verticalCut.length == n - 1
//     1 <= horizontalCut[i], verticalCut[i] <= 10^3
//

struct Solution;

impl Solution {
    pub fn minimum_cost(_m: i32, _n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64 {
        let (mut hc, mut vc) = (1, 1);
        let mut heap = std::collections::BinaryHeap::new();
        for &c in &horizontal_cut {
            heap.push((c, false));
        }
        for &c in &vertical_cut {
            heap.push((c, true));
        }
        let mut res = 0;
        while let Some(t) = heap.pop() {
            if t.1 {
                res += vc * t.0 as i64;
                hc += 1;
            } else {
                res += hc * t.0 as i64;
                vc += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let m = 3;
    let n = 2;
    let horizontal_cut = vec![1, 3];
    let vertical_cut = vec![5];
    assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), 13);

    let m = 2;
    let n = 2;
    let horizontal_cut = vec![7];
    let vertical_cut = vec![4];
    assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), 15);
}
