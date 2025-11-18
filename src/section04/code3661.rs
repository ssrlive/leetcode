#![allow(dead_code)]

// 3661. Maximum Walls Destroyed by Robots
// https://leetcode.com/problems/maximum-walls-destroyed-by-robots/
// https://leetcode.cn/problems/maximum-walls-destroyed-by-robots/
//
// Hard
//
// There is an endless straight line populated with some robots and walls. You are given integer arrays robots, distance, and walls:
// robots[i] is the position of the ith robot.
// distance[i] is the maximum distance the ith robot's bullet can travel.
// walls[j] is the position of the jth wall.
// Every robot has one bullet that can either fire to the left or the right at most distance[i] meters.
//
// A bullet destroys every wall in its path that lies within its range. Robots are fixed obstacles: if a bullet hits another robot before reaching a wall, it immediately stops at that robot and cannot continue.
//
// Return the maximum number of unique walls that can be destroyed by the robots.
//
// Notes:
//
// A wall and a robot may share the same position; the wall can be destroyed by the robot at that position.
// Robots are not destroyed by bullets.
//
// Example 1:
//
// Input: robots = [4], distance = [3], walls = [1,10]
//
// Output: 1
//
// Explanation:
//
// robots[0] = 4 fires left with distance[0] = 3, covering [1, 4] and destroys walls[0] = 1.
// Thus, the answer is 1.
//
// Example 2:
//
// Input: robots = [10,2], distance = [5,1], walls = [5,2,7]
//
// Output: 3
//
// Explanation:
//
// robots[0] = 10 fires left with distance[0] = 5, covering [5, 10] and destroys walls[0] = 5 and walls[2] = 7.
// robots[1] = 2 fires left with distance[1] = 1, covering [1, 2] and destroys walls[1] = 2.
// Thus, the answer is 3.
//
// Example 3:
// Input: robots = [1,2], distance = [100,1], walls = [10]
//
// Output: 0
//
// Explanation:
//
// In this example, only robots[0] can reach the wall, but its shot to the right is blocked by robots[1]; thus the answer is 0.
//
// Constraints:
//
// 1 <= robots.length == distance.length <= 10^5
// 1 <= walls.length <= 10^5
// 1 <= robots[i], walls[j] <= 10^9
// 1 <= distance[i] <= 10^5
// All values in robots are unique
// All values in walls are unique
//

struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let mut x = vec![(0, 0); n];
        for i in 0..n {
            x[i] = (robots[i], distance[i]);
        }
        let mut walls = walls;
        walls.sort_unstable();
        x.sort_unstable();
        x.push((1_000_000_000, 0));

        // finds the no of walls in the range [l,r];
        let query = |l: i32, r: i32| -> i32 {
            if l > r {
                return 0;
            }
            let it1 = walls.partition_point(|&x| x <= r);
            let it2 = walls.partition_point(|&x| x < l);
            (it1 - it2) as i32
        };
        let mut dp = vec![(0, 0); n];
        // base case
        dp[0].0 = query(x[0].0 - x[0].1, x[0].0);
        if n > 1 {
            dp[0].1 = query(x[0].0, (x[1].0 - 1).min(x[0].0 + x[0].1));
        } else {
            dp[0].1 = query(x[0].0, x[0].0 + x[0].1);
        }
        //transition
        for i in 1..n {
            dp[i].1 = dp[i - 1].0.max(dp[i - 1].1) + query(x[i].0, (x[i + 1].0 - 1).min(x[i].0 + x[i].1));

            dp[i].0 = dp[i - 1].0 + query((x[i].0 - x[i].1).max(x[i - 1].0 + 1), x[i].0);
            let res = dp[i - 1].1 + query((x[i].0 - x[i].1).max(x[i - 1].0 + 1), x[i].0)
                - query((x[i].0 - x[i].1).max(x[i - 1].0 + 1), (x[i - 1].0 + x[i - 1].1).min(x[i].0 - 1));
            dp[i].0 = dp[i].0.max(res);
        }
        dp[n - 1].0.max(dp[n - 1].1)
    }
}

#[test]
fn test() {
    let robots = vec![4];
    let distance = vec![3];
    let walls = vec![1, 10];
    assert_eq!(Solution::max_walls(robots, distance, walls), 1);

    let robots = vec![10, 2];
    let distance = vec![5, 1];
    let walls = vec![5, 2, 7];
    assert_eq!(Solution::max_walls(robots, distance, walls), 3);

    let robots = vec![1, 2];
    let distance = vec![100, 1];
    let walls = vec![10];
    assert_eq!(Solution::max_walls(robots, distance, walls), 0);

    let robots = vec![17, 59, 32, 11, 72, 18];
    let distance = vec![5, 7, 6, 5, 2, 10];
    let walls = vec![
        17, 25, 33, 29, 54, 53, 18, 35, 39, 37, 20, 14, 34, 13, 16, 58, 22, 51, 56, 27, 10, 15, 12, 23, 45, 43, 21, 2, 42, 7, 32, 40, 8, 9,
        1, 5, 55, 30, 38, 4, 3, 31, 36, 41, 57, 28, 11, 49, 26, 19, 50, 52, 6, 47, 46, 44, 24, 48,
    ];
    assert_eq!(Solution::max_walls(robots, distance, walls), 37);
}
