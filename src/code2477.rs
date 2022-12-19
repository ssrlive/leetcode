#![allow(dead_code)]

// 2477. Minimum Fuel Cost to Report to the Capital
// https://leetcode.com/problems/minimum-fuel-cost-to-report-to-the-capital/
// https://leetcode.cn/problems/minimum-fuel-cost-to-report-to-the-capital/
//
// There is a tree (i.e., a connected, undirected graph with no cycles) structure country network consisting of n cities
// numbered from 0 to n - 1 and exactly n - 1 roads. The capital city is city 0. You are given a 2D integer array roads
// where roads[i] = [ai, bi] denotes that there exists a bidirectional road connecting cities ai and bi.
//
// There is a meeting for the representatives of each city. The meeting is in the capital city.
//
// There is a car in each city. You are given an integer seats that indicates the number of seats in each car.
//
// A representative can use the car in their city to travel or change the car and ride with another representative. The cost of traveling between two cities is one liter of fuel.
//
// Return the minimum number of liters of fuel to reach the capital city.
//
// Example 1:
//
// Input: roads = [[0,1],[0,2],[0,3]], seats = 5
// Output: 3
// Explanation:
// - Representative1 goes directly to the capital with 1 liter of fuel.
// - Representative2 goes directly to the capital with 1 liter of fuel.
// - Representative3 goes directly to the capital with 1 liter of fuel.
// It costs 3 liters of fuel at minimum.
// It can be proven that 3 is the minimum number of liters of fuel needed.
//
// Example 2:
//
// Input: roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2
// Output: 7
// Explanation:
// - Representative2 goes directly to city 3 with 1 liter of fuel.
// - Representative2 and representative3 go together to city 1 with 1 liter of fuel.
// - Representative2 and representative3 go together to the capital with 1 liter of fuel.
// - Representative1 goes directly to the capital with 1 liter of fuel.
// - Representative5 goes directly to the capital with 1 liter of fuel.
// - Representative6 goes directly to city 4 with 1 liter of fuel.
// - Representative4 and representative6 go together to the capital with 1 liter of fuel.
// It costs 7 liters of fuel at minimum.
// It can be proven that 7 is the minimum number of liters of fuel needed.
//
// Example 3:
//
// Input: roads = [], seats = 1
// Output: 0
// Explanation: No representatives need to travel to the capital city.
//
// Constraints:
//
// - 1 <= n <= 10^5
// - roads.length == n - 1
// - roads[i].length == 2
// - 0 <= ai, bi < n
// - ai != bi
// - roads represents a valid tree.
// - 1 <= seats <= 10^5
//

struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        use std::collections::HashMap;
        fn dfs(
            adj: &HashMap<usize, Vec<usize>>,
            curr: usize,
            visited: &mut Vec<bool>,
            seats: i32,
            ans: &mut i64,
        ) -> Option<i32> {
            visited[curr] = true;
            let mut count = 1;
            let v = adj.get(&curr)?;
            for &x in v.iter() {
                if !visited[x] {
                    count += dfs(adj, x, visited, seats, ans)?;
                }
            }
            let mut p = count / seats;
            if count % seats != 0 {
                p += 1;
            }
            if curr != 0 {
                *ans += p as i64;
            }
            Some(count)
        }

        let mut ans = 0;
        let mut adj = HashMap::<usize, Vec<usize>>::new();
        if roads.is_empty() {
            return 0;
        }
        for x in roads.iter() {
            adj.entry(x[0] as usize).or_default().push(x[1] as usize);
            adj.entry(x[1] as usize).or_default().push(x[0] as usize);
        }
        let mut visited = vec![false; roads.len() + 1];
        dfs(&adj, 0, &mut visited, seats, &mut ans);
        ans
    }
}

#[test]
fn test() {
    let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    assert_eq!(Solution::minimum_fuel_cost(roads, 5), 3);
    let roads = vec![vec![3, 1], vec![3, 2], vec![1, 0], vec![0, 4], vec![0, 5], vec![4, 6]];
    assert_eq!(Solution::minimum_fuel_cost(roads, 2), 7);
    assert_eq!(Solution::minimum_fuel_cost(vec![], 1), 0);
}
