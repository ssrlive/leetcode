#![allow(dead_code)]

// 3218. Minimum Cost for Cutting Cake I
// https://leetcode.com/problems/minimum-cost-for-cutting-cake-i/
// https://leetcode.cn/problems/minimum-cost-for-cutting-cake-i/
//
// Medium
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
//     1 <= m, n <= 20
//     horizontalCut.length == m - 1
//     verticalCut.length == n - 1
//     1 <= horizontalCut[i], verticalCut[i] <= 10^3
//

struct Solution;

impl Solution {
    pub fn minimum_cost(_m: i32, _n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
        let mut h_cnt = 1i32;
        let mut v_cnt = 1i32;
        let mut cost = 0i32;
        let mut heap = horizontal_cut
            .iter()
            .map(|v| (*v, b'h'))
            .chain(vertical_cut.iter().map(|v| (*v, b'v')))
            .collect::<std::collections::BinaryHeap<(i32, u8)>>();

        while let Some((val, dir)) = heap.pop() {
            if dir.eq(&b'h') {
                cost += val * v_cnt;
                h_cnt += 1;
            } else {
                cost += val * h_cnt;
                v_cnt += 1;
            }
        }

        cost
    }
}

#[test]
fn test() {
    let m = 3;
    let n = 2;
    let horizontal_cut = vec![1, 3];
    let vertical_cut = vec![5];
    let output = 13;
    assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), output);

    let m = 2;
    let n = 2;
    let horizontal_cut = vec![7];
    let vertical_cut = vec![4];
    let output = 15;
    assert_eq!(Solution::minimum_cost(m, n, horizontal_cut, vertical_cut), output);
}
