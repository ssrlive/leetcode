#![allow(dead_code)]

// 1184. Distance Between Bus Stops
// https://leetcode.com/problems/distance-between-bus-stops/
// https://leetcode.cn/problems/distance-between-bus-stops/
//
// A bus has n stops numbered from 0 to n - 1 that form a circle. We know the distance between all pairs of
// neighboring stops where distance[i] is the distance between the stops number i and (i + 1) % n.
//
// The bus goes along both directions i.e. clockwise and counterclockwise.
//
// Return the shortest distance between the given start and destination stops.
//
// Example 1:
//
// Input: distance = [1,2,3,4], start = 0, destination = 1
// Output: 1
// Explanation: Distance between 0 and 1 is 1 or 9, minimum is 1.
//
// Example 2:
//
// Input: distance = [1,2,3,4], start = 0, destination = 2
// Output: 3
// Explanation: Distance between 0 and 2 is 3 or 7, minimum is 3.
//
// Example 3:
//
// Input: distance = [1,2,3,4], start = 0, destination = 3
// Output: 4
// Explanation: Distance between 0 and 3 is 6 or 4, minimum is 4.
//
// Constraints:
//
// - 1 <= n <= 10^4
// - distance.length == n
// - 0 <= start, destination < n
// - 0 <= distance[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (s0, s1) = match start <= destination {
            true => (start as usize, destination as usize),
            false => (destination as usize, start as usize),
        };

        distance[s0..s1]
            .iter()
            .sum::<i32>()
            .min(distance[..s0].iter().sum::<i32>() + distance[s1..].iter().sum::<i32>())
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], 0, 1, 1),
        (vec![1, 2, 3, 4], 0, 2, 3),
        (vec![1, 2, 3, 4], 0, 3, 4),
    ];
    for (dist, s, dest, ex) in cases {
        assert_eq!(Solution::distance_between_bus_stops(dist, s, dest), ex);
    }
}
