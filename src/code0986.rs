#![allow(dead_code)]

// 986. Interval List Intersections
// https://leetcode.com/problems/interval-list-intersections/
// https://leetcode.cn/problems/interval-list-intersections/
//
// You are given two lists of closed intervals, firstList and secondList, where firstList[i] = [starti, endi]
// and secondList[j] = [startj, endj]. Each list of intervals is pairwise disjoint and in sorted order.
//
// Return the intersection of these two interval lists.
//
// A closed interval [a, b] (with a <= b) denotes the set of real numbers x with a <= x <= b.
//
// The intersection of two closed intervals is a set of real numbers that are either empty or represented as a closed interval.
// For example, the intersection of [1, 3] and [2, 4] is [2, 3].
//
// Example 1:
//
// Input: firstList = [[0,2],[5,10],[13,23],[24,25]], secondList = [[1,5],[8,12],[15,24],[25,26]]
// Output: [[1,2],[5,5],[8,10],[15,23],[24,24],[25,25]]
//
// Example 2:
//
// Input: firstList = [[1,3],[5,9]], secondList = []
// Output: []
//
// Constraints:
//
// - 0 <= firstList.length, secondList.length <= 1000
// - firstList.length + secondList.length >= 1
// - 0 <= starti < endi <= 10^9
// - endi < starti+1
// - 0 <= startj < endj <= 10^9
// - endj < startj+1
//

struct Solution;

impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::{max, min};
        const START: usize = 0;
        const END: usize = 1;

        fn overlap(u: &[i32], v: &[i32]) -> bool {
            u[START] <= v[END] && v[START] <= u[END]
        }

        fn intersect(u: &[i32], v: &[i32]) -> Vec<i32> {
            vec![max(u[START], v[START]), min(u[END], v[END])]
        }

        let mut intersection = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < first_list.len() && j < second_list.len() {
            if overlap(&first_list[i], &second_list[j]) {
                intersection.push(intersect(&first_list[i], &second_list[j]));
            }
            if first_list[i][END] < second_list[j][END] {
                i += 1;
            } else {
                j += 1;
            }
        }
        intersection
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]],
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25],
            ],
        ),
        (vec![vec![1, 3], vec![5, 9]], vec![], vec![]),
    ];
    for (first_list, second_list, expected) in cases {
        assert_eq!(Solution::interval_intersection(first_list, second_list), expected);
    }
}
