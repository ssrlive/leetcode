#![allow(dead_code)]

// 787. Cheapest Flights Within K Stops
// https://leetcode.com/problems/cheapest-flights-within-k-stops/
// https://leetcode.cn/problems/cheapest-flights-within-k-stops/
//
// There are n cities connected by some number of flights. You are given an array flights where flights[i] = [fromi, toi, pricei]
// indicates that there is a flight from city fromi to city toi with cost pricei.
//
// You are also given three integers src, dst, and k, return the cheapest price
// from src to dst with at most k stops. If there is no such route, return -1.
//
// Example 1:
//
// Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
// Output: 700
// Explanation:
// The graph is shown above.
// The optimal path with at most 1 stop from city 0 to 3 is marked in red and has cost 100 + 600 = 700.
// Note that the path through cities [0,1,2,3] is cheaper but is invalid because it uses 2 stops.
//
// Example 2:
//
// Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
// Output: 200
// Explanation:
// The graph is shown above.
// The optimal path with at most 1 stop from city 0 to 2 is marked in red and has cost 100 + 100 = 200.
//
// Example 3:
//
// Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
// Output: 500
// Explanation:
// The graph is shown above.
// The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.
//
// Constraints:
//
// - 1 <= n <= 100
// - 0 <= flights.length <= (n * (n - 1) / 2)
// - flights[i].length == 3
// - 0 <= fromi, toi < n
// - fromi != toi
// - 1 <= pricei <= 10^4
// - There will not be any multiple flights between two cities.
// - 0 <= src, dst, k < n
// - src != dst
//

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let (src, dst) = (src as usize, dst as usize);
        let (n, k) = (n as usize, k as usize);
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

        for flight in flights {
            graph[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }

        let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX; k + 1]; n];

        let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();

        dist[src].iter_mut().for_each(|cell| *cell = 0);
        pq.push((0, src, 0));
        while let Some((d, u, cnt)) = pq.pop() {
            if u == dst {
                return -d;
            }
            if cnt == k + 1 {
                continue;
            }

            for p in &graph[u] {
                if dist[p.0][cnt] <= p.1 - d {
                    continue;
                }
                dist[p.0][cnt] = p.1 - d;
                pq.push((d - p.1, p.0, cnt + 1));
            }
        }

        -1
    }
}

#[test]
fn test() {
    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 0, 100], vec![1, 3, 600], vec![2, 3, 200]];
    assert_eq!(Solution::find_cheapest_price(4, flights, 0, 3, 1), 700);

    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 1), 200);

    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 0), 500);
}
