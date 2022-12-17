#![allow(dead_code)]

// 815. Bus Routes
// https://leetcode.com/problems/bus-routes/
//
// You are given an array routes representing bus routes where routes[i] is a bus route that the ith bus repeats forever.
//
// - For example, if routes[0] = [1, 5, 7], this means that the 0th bus travels in the sequence 1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... forever.
//
// You will start at the bus stop source (You are not on any bus initially), and you want to go to the bus stop target. You can travel between bus stops by buses only.
//
// Return the least number of buses you must take to travel from source to target. Return -1 if it is not possible.
//
// Example 1:
//
// Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
// Output: 2
// Explanation: The best strategy is take the first bus to the bus stop 7, then take the second bus to the bus stop 6.
//
// Example 2:
//
// Input: routes = [[7,12],[4,5,15],[6],[15,19],[9,12,13]], source = 15, target = 12
// Output: -1
//
// Constraints:
//
// - 1 <= routes.length <= 500.
// - 1 <= routes[i].length <= 10^5
// - All the values of routes[i] are unique.
// - sum(routes[i].length) <= 10^5
// - 0 <= routes[i][j] < 10^6
// - 0 <= source, target < 10^6
//

struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};
        let mut bus_stop_bus_route = HashMap::new();
        for (route_index, bus_route) in routes.iter().enumerate() {
            for bus_stop in bus_route {
                bus_stop_bus_route
                    .entry(*bus_stop)
                    .or_insert_with(HashSet::new)
                    .insert(route_index);
            }
        }

        let mut visited_bus_stop = HashSet::new();
        visited_bus_stop.insert(source);
        let mut visited_route = HashSet::new();
        let num_buses = 0;

        let mut journey = VecDeque::new();
        journey.push_back((source, num_buses));

        while let Some((bus_stop, num_buses)) = journey.pop_front() {
            if bus_stop == target {
                return num_buses;
            }
            for next_route in bus_stop_bus_route[&bus_stop].iter() {
                if !visited_route.contains(next_route) {
                    visited_route.insert(*next_route);
                    for new_bus_stop in routes[*next_route].iter() {
                        if !visited_bus_stop.contains(new_bus_stop) {
                            visited_bus_stop.insert(*new_bus_stop);
                            journey.push_back((*new_bus_stop, num_buses + 1));
                        }
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
    let source = 1;
    let target = 6;
    assert_eq!(Solution::num_buses_to_destination(routes, source, target), 2);

    let routes = vec![vec![7, 12], vec![4, 5, 15], vec![6], vec![15, 19], vec![9, 12, 13]];
    let source = 15;
    let target = 12;
    assert_eq!(Solution::num_buses_to_destination(routes, source, target), -1);
}
