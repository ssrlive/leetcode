#![allow(dead_code)]

// 3342. Find Minimum Time to Reach Last Room II
// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii/
// https://leetcode.cn/problems/find-minimum-time-to-reach-last-room-ii/
//
// Medium
//
// There is a dungeon with n x m rooms arranged as a grid.
//
// You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds
// when you can start moving to that room. You start from the room (0, 0) at time t = 0 and can move to an adjacent room.
// Moving between adjacent rooms takes one second for one move and two seconds for the next, alternating between the two.
//
// Return the minimum time to reach the room (n - 1, m - 1).
//
// Two rooms are adjacent if they share a common wall, either horizontally or vertically.
//
// Example 1:
//
// Input: moveTime = [[0,4],[4,4]]
//
// Output: 7
//
// Explanation:
//
// The minimum time required is 7 seconds.
//
//     At time t == 4, move from room (0, 0) to room (1, 0) in one second.
//     At time t == 5, move from room (1, 0) to room (1, 1) in two seconds.
//
// Example 2:
//
// Input: moveTime = [[0,0,0,0],[0,0,0,0]]
//
// Output: 6
//
// Explanation:
//
// The minimum time required is 6 seconds.
//
//     At time t == 0, move from room (0, 0) to room (1, 0) in one second.
//     At time t == 1, move from room (1, 0) to room (1, 1) in two seconds.
//     At time t == 3, move from room (1, 1) to room (1, 2) in one second.
//     At time t == 4, move from room (1, 2) to room (1, 3) in two seconds.
//
// Example 3:
//
// Input: moveTime = [[0,1],[1,2]]
//
// Output: 4
//
// Constraints:
//
//     2 <= n == moveTime.length <= 750
//     2 <= m == moveTime[i].length <= 750
//     0 <= moveTime[i][j] <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let move_time: Vec<Vec<i64>> = move_time.iter().map(|v| v.iter().map(|&x| x as i64).collect()).collect();
        let mut pq = std::collections::BinaryHeap::new();

        let n = move_time.len() as i64;
        let m = move_time[0].len() as i64;

        let t = vec![1e15; m as usize];
        let mut dist = vec![t; n as usize];

        // initially dec = 0 as time to move = 1.
        pq.push((0, 0, 0, 0));
        dist[0][0] = 0.0;

        let (mut ans, mut _ex, mut _nx) = (0, 0, 0);

        let xm = [1, -1, 0, 0];
        let ym = [0, 0, 1, -1];

        while let Some((d, x, y, dec)) = pq.pop() {
            // take negetive to get the actual value. While pushing we use negetive sign to maintain min heap.
            let d = -d;

            if dec == 0 {
                _nx = 1;
                _ex = 1;
            } else {
                _nx = 0;
                _ex = 2;
            }

            if dist[x as usize][y as usize] < d as f64 {
                continue;
            }

            if x == n - 1 && y == m - 1 {
                ans = d;
                break;
            }

            for i in 0..4 {
                let xn = x + xm[i];
                let yn = y + ym[i];

                if xn >= 0 && xn < n && yn >= 0 && yn < m {
                    let a = move_time[xn as usize][yn as usize];
                    // adding time required to move ex.
                    let now = std::cmp::max(d + _ex, a + _ex);

                    if dist[xn as usize][yn as usize] > now as f64 {
                        dist[xn as usize][yn as usize] = now as f64;
                        // push negetive value of now to convert default max heap to min heap and push toggled dec variable nx.
                        pq.push((-now, xn, yn, _nx));
                    }
                }
            }
        }

        ans as i32
    }
}

#[test]
fn test() {
    let move_time = vec![vec![0, 4], vec![4, 4]];
    let res = 7;
    assert_eq!(Solution::min_time_to_reach(move_time), res);

    let move_time = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
    let res = 6;
    assert_eq!(Solution::min_time_to_reach(move_time), res);

    let move_time = vec![vec![0, 1], vec![1, 2]];
    let res = 4;
    assert_eq!(Solution::min_time_to_reach(move_time), res);
}
