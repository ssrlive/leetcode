#![allow(dead_code)]

// 134. Gas Station
// https://leetcode.com/problems/gas-station/
// https://leetcode.cn/problems/gas-station/
//
// There are N gas stations along a circular route, where the amount of gas at station i is gas[i].
//
// You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from station i to its next station (i+1). You begin the journey with an empty tank at one of the gas stations.
//
// Given two integer arrays gas and cost, return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1. If there exists a solution, it is guaranteed to be unique
//

struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut total = 0;
        let mut tank = 0;
        for i in 0..gas.len() {
            tank += gas[i] - cost[i];
            if tank < 0 {
                start = i + 1;
                total += tank;
                tank = 0;
            }
        }
        if total + tank < 0 { -1 } else { start as i32 }
    }
}

#[test]
fn test_can_complete_circuit() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}
