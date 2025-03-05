#![allow(dead_code)]

// 365. Water and Jug Problem
// https://leetcode.com/problems/water-and-jug-problem/
// https://leetcode.cn/problems/water-and-jug-problem/
//
// You are given two jugs with capacities jug1Capacity and jug2Capacity liters. There is an infinite amount of water supply available.
// Determine whether it is possible to measure exactly targetCapacity liters using these two jugs.
//
// If targetCapacity liters of water are measurable, you must have targetCapacity liters of water contained within one or both buckets by the end.
//
// Operations allowed:
//
// - Fill any of the jugs with water.
// - Empty any of the jugs.
// - Pour water from one jug into another till the other jug is completely full, or the first jug itself is empty.
//
// Example 1:
//
// Input: jug1Capacity = 3, jug2Capacity = 5, targetCapacity = 4
// Output: true
// Explanation: The famous Die Hard example
//
// Example 2:
//
// Input: jug1Capacity = 2, jug2Capacity = 6, targetCapacity = 5
// Output: false
//
// Example 3:
//
// Input: jug1Capacity = 1, jug2Capacity = 2, targetCapacity = 3
// Output: true
//
// Constraints:
//
// - 1 <= jug1Capacity, jug2Capacity, targetCapacity <= 10^6
//

struct Solution;

// use std::collections::HashSet;
impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            let (a, b) = if a > b { (a, b) } else { (b, a) };
            if b == 0 { a } else { gcd(b, a % b) }
        }
        jug2_capacity + jug1_capacity >= target_capacity && target_capacity % gcd(jug1_capacity, jug2_capacity) == 0
    }
}

#[test]
fn test() {
    assert!(Solution::can_measure_water(3, 5, 4));
    assert!(!Solution::can_measure_water(2, 6, 5));
    assert!(Solution::can_measure_water(1, 2, 3));
    assert!(!Solution::can_measure_water(1, 1, 12));
    assert!(!Solution::can_measure_water(2, 6, 5));
    assert!(Solution::can_measure_water(22003, 31237, 1));
    assert!(Solution::can_measure_water(104579, 104593, 12444));
}
