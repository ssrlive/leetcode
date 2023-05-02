#![allow(dead_code)]

/*
// 2662. Minimum Cost of a Path With Special Roads
// https://leetcode.com/problems/minimum-cost-of-a-path-with-special-roads/
// https://leetcode.cn/problems/minimum-cost-of-a-path-with-special-roads/
//
// Medium
//
// You are given an array start where start = [startX, startY] represents your initial position (startX, startY) in a 2D space.
// You are also given the array target where target = [targetX, targetY] represents your target position (targetX, targetY).

The cost of going from a position (x1, y1) to any other position in the space (x2, y2) is |x2 - x1| + |y2 - y1|.

There are also some special roads. You are given a 2D array specialRoads where specialRoads[i] = [x1i, y1i, x2i, y2i, costi] indicates that the ith special road can take you from (x1i, y1i) to (x2i, y2i) with a cost equal to costi. You can use each special road any number of times.

Return the minimum cost required to go from (startX, startY) to (targetX, targetY).

Example 1:

Input: start = [1,1], target = [4,5], specialRoads = [[1,2,3,3,2],[3,4,4,5,1]]
Output: 5
Explanation: The optimal path from (1,1) to (4,5) is the following:
- (1,1) -> (1,2). This move has a cost of |1 - 1| + |2 - 1| = 1.
- (1,2) -> (3,3). This move uses the first special edge, the cost is 2.
- (3,3) -> (3,4). This move has a cost of |3 - 3| + |4 - 3| = 1.
- (3,4) -> (4,5). This move uses the second special edge, the cost is 1.
So the total cost is 1 + 2 + 1 + 1 = 5.
It can be shown that we cannot achieve a smaller total cost than 5.

Example 2:

Input: start = [3,2], target = [5,7], specialRoads = [[3,2,3,4,4],[3,3,5,5,5],[3,4,5,6,6]]
Output: 7
Explanation: It is optimal to not use any special edges and go directly from the starting to the ending position with a cost |5 - 3| + |7 - 2| = 7.

Constraints:

    start.length == target.length == 2
    1 <= startX <= targetX <= 10^5
    1 <= startY <= targetY <= 10^5
    1 <= specialRoads.length <= 200
    specialRoads[i].length == 5
    startX <= x1i, x2i <= targetX
    startY <= y1i, y2i <= targetY
    1 <= costi <= 10^5
*/

struct Solution;

impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
        let mut special_roads = special_roads;
        let mut res = target[0] - start[0] + target[1] - start[1];
        let sz = special_roads.len();
        special_roads.push(vec![start[0], start[1], start[0], start[1], 0]);
        let mut dp = vec![std::i32::MAX; sz + 1];
        let mut q = vec![vec![sz as i32, 0]];
        let mut q1 = vec![];
        while !q.is_empty() {
            q1.clear();
            for qq in q.iter() {
                let (i, cost_i) = (qq[0] as usize, qq[1]);
                if cost_i <= dp[i] {
                    let x2i = special_roads[i][2];
                    let y2i = special_roads[i][3];
                    for j in 0..sz {
                        let x1j = special_roads[j][0];
                        let y1j = special_roads[j][1];
                        let x2j = special_roads[j][2];
                        let y2j = special_roads[j][3];
                        let cost_j = special_roads[j][4];
                        dp[j] = std::cmp::min(dp[j], cost_i + (x2j - x2i).abs() + (y2j - y2i).abs());
                        let take = cost_i + cost_j + (x1j - x2i).abs() + (y1j - y2i).abs();
                        if take < dp[j] {
                            dp[j] = take;
                            q1.push(vec![j as i32, take]);
                            res = std::cmp::min(res, take + (x2j - target[0]).abs() + (y2j - target[1]).abs());
                        }
                    }
                }
            }
            std::mem::swap(&mut q, &mut q1);
        }
        res
    }
}

#[test]
fn test() {
    let start = vec![1, 1];
    let target = vec![4, 5];
    let special_roads = vec![vec![1, 2, 3, 3, 2], vec![3, 4, 4, 5, 1]];
    let res = 5;
    assert_eq!(Solution::minimum_cost(start, target, special_roads), res);

    let start = vec![3, 2];
    let target = vec![5, 7];
    let special_roads = vec![vec![3, 2, 3, 4, 4], vec![3, 3, 5, 5, 5], vec![3, 4, 5, 6, 6]];
    let res = 7;
    assert_eq!(Solution::minimum_cost(start, target, special_roads), res);
}
