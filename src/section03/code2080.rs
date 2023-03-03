#![allow(dead_code)]

/*

// 2080. Range Frequency Queries
// https://leetcode.com/problems/range-frequency-queries/
// https://leetcode.cn/problems/range-frequency-queries/
//
// Medium
//
// Design a data structure to find the frequency of a given value in a given subarray.

The frequency of a value in a subarray is the number of occurrences of that value in the subarray.

Implement the RangeFreqQuery class:

    RangeFreqQuery(int[] arr) Constructs an instance of the class with the given 0-indexed integer array arr.
    int query(int left, int right, int value) Returns the frequency of value in the subarray arr[left...right].

A subarray is a contiguous sequence of elements within an array. arr[left...right] denotes the subarray that contains the elements of nums between indices left and right (inclusive).

Example 1:

Input
["RangeFreqQuery", "query", "query"]
[[[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]], [1, 2, 4], [0, 11, 33]]
Output
[null, 1, 2]

Explanation
RangeFreqQuery rangeFreqQuery = new RangeFreqQuery([12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
rangeFreqQuery.query(1, 2, 4); // return 1. The value 4 occurs 1 time in the subarray [33, 4]
rangeFreqQuery.query(0, 11, 33); // return 2. The value 33 occurs 2 times in the whole array.

Constraints:

    1 <= arr.length <= 10^5
    1 <= arr[i], value <= 10^4
    0 <= left <= right < arr.length
    At most 105 calls will be made to query
*/

use std::collections::HashMap;

pub struct RangeFreqQuery {
    freq: HashMap<i32, Vec<i32>>,
}

impl RangeFreqQuery {
    pub fn new(arr: Vec<i32>) -> Self {
        let mut freq = HashMap::new();
        for (i, &arr_i) in arr.iter().enumerate() {
            freq.entry(arr_i).or_insert(vec![]).push(i as i32);
        }
        RangeFreqQuery { freq }
    }

    pub fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        match self.freq.get(&value) {
            Some(indices) => {
                let l = match indices.binary_search(&left) {
                    Ok(i) => i,
                    Err(i) => i,
                } as i32;
                let r = match indices.binary_search(&right) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                } as i32;
                r - l
            }
            _ => 0,
        }
    }
}

#[test]
fn test() {
    let range_freq_query = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
    assert_eq!(range_freq_query.query(1, 2, 4), 1);
    assert_eq!(range_freq_query.query(0, 11, 33), 2);
}
