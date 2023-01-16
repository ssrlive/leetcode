#![allow(dead_code)]

// 1157. Online Majority Element In Subarray
// https://leetcode.com/problems/online-majority-element-in-subarray/
// https://leetcode.cn/problems/online-majority-element-in-subarray/
//
// Design a data structure that efficiently finds the majority element of a given subarray.
//
// The majority element of a subarray is an element that occurs threshold times or more in the subarray.
//
// Implementing the MajorityChecker class:
//
// MajorityChecker(int[] arr) Initializes the instance of the class with the given array arr.
// int query(int left, int right, int threshold) returns the element in the subarray arr[left...right] that occurs at least threshold times, or -1 if no such element exists.
//
// Example 1:
//
// Input
// ["MajorityChecker", "query", "query", "query"]
// [[[1, 1, 2, 2, 1, 1]], [0, 5, 4], [0, 3, 3], [2, 3, 2]]
// Output
// [null, 1, -1, 2]
//
// Explanation
// MajorityChecker majorityChecker = new MajorityChecker([1, 1, 2, 2, 1, 1]);
// majorityChecker.query(0, 5, 4); // return 1
// majorityChecker.query(0, 3, 3); // return -1
// majorityChecker.query(2, 3, 2); // return 2
//
// Constraints:
//
// - 1 <= arr.length <= 2 * 10^4
// - 1 <= arr[i] <= 2 * 10^4
// - 0 <= left <= right < arr.length
// - threshold <= right - left + 1
// - 2 * threshold > right - left + 1
// - At most 104 calls will be made to query.
//

use rand::prelude::*;
use std::collections::HashMap;

struct MajorityChecker {
    pos: HashMap<i32, Vec<usize>>,
    a: Vec<i32>,
    try_bound: usize,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut pos = HashMap::<i32, Vec<_>>::new();
        for (i, &num) in arr.iter().enumerate() {
            pos.entry(num).or_default().push(i);
        }
        Self {
            pos,
            a: arr,
            try_bound: 20,
        }
    }

    fn get_occurrence(&self, num: i32, l: usize, r: usize) -> Option<usize> {
        let vec = self.pos.get(&num)?;
        let iter_l = vec.iter().position(|&x| x >= l)?;
        let iter_r = vec.iter().position(|&x| x > r).unwrap_or(vec.len());
        Some(iter_r - iter_l)
    }

    fn get_random(&self, l: usize, r: usize) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(l..(r + 1))
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let threshold = threshold as usize;
        for _ in 0..self.try_bound {
            let index = self.get_random(left, right);
            let elem = self.a[index];
            if self.get_occurrence(elem, left, right).unwrap_or(0) >= threshold {
                return elem;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let checker = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    assert_eq!(checker.query(0, 5, 4), 1);
    assert_eq!(checker.query(0, 3, 3), -1);
    assert_eq!(checker.query(2, 3, 2), 2);
}
