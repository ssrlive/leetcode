#![allow(dead_code)]

/*

// 1825. Finding MK Average
// https://leetcode.com/problems/finding-mk-average/
// https://leetcode.cn/problems/finding-mk-average/
//
// Hard
//
You are given two integers, m and k, and a stream of integers. You are tasked to implement a data structure that calculates the MKAverage for the stream.

The MKAverage can be calculated using these steps:

    If the number of the elements in the stream is less than m you should consider the MKAverage to be -1. Otherwise, copy the last m elements of the stream to a separate container.
    Remove the smallest k elements and the largest k elements from the container.
    Calculate the average value for the rest of the elements rounded down to the nearest integer.

Implement the MKAverage class:

    MKAverage(int m, int k) Initializes the MKAverage object with an empty stream and the two integers m and k.
    void addElement(int num) Inserts a new element num into the stream.
    int calculateMKAverage() Calculates and returns the MKAverage for the current stream rounded down to the nearest integer.

Example 1:

Input
["MKAverage", "addElement", "addElement", "calculateMKAverage", "addElement", "calculateMKAverage", "addElement", "addElement", "addElement", "calculateMKAverage"]
[[3, 1], [3], [1], [], [10], [], [5], [5], [5], []]
Output
[null, null, null, -1, null, 3, null, null, null, 5]

Explanation
MKAverage obj = new MKAverage(3, 1);
obj.addElement(3);        // current elements are [3]
obj.addElement(1);        // current elements are [3,1]
obj.calculateMKAverage(); // return -1, because m = 3 and only 2 elements exist.
obj.addElement(10);       // current elements are [3,1,10]
obj.calculateMKAverage(); // The last 3 elements are [3,1,10].
                          // After removing smallest and largest 1 element the container will be [3].
                          // The average of [3] equals 3/1 = 3, return 3
obj.addElement(5);        // current elements are [3,1,10,5]
obj.addElement(5);        // current elements are [3,1,10,5,5]
obj.addElement(5);        // current elements are [3,1,10,5,5,5]
obj.calculateMKAverage(); // The last 3 elements are [5,5,5].
                          // After removing smallest and largest 1 element the container will be [5].
                          // The average of [5] equals 5/1 = 5, return 5

Constraints:

    3 <= m <= 10^5
    1 <= k*2 < m
    1 <= num <= 10^5
    At most 105 calls will be made to addElement and calculateMKAverage.
*/

use std::collections::{BTreeMap, VecDeque};
#[allow(unused)]
struct MKAverage {
    m: usize,
    k: usize,
    m_sum: i32,
    stream: VecDeque<i32>,
    sort: BTreeMap<i32, i32>,
}

#[allow(unused)]
impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        MKAverage {
            m: m as usize,
            k: k as usize,
            m_sum: 0,
            stream: VecDeque::new(),
            sort: BTreeMap::new(),
        }
    }

    fn add_element(&mut self, num: i32) {
        self.stream.push_back(num);
        *self.sort.entry(num).or_insert(0) += 1;
        self.m_sum += num;
        if self.stream.len() > self.m {
            let rem = self.stream.pop_front().unwrap();
            *self.sort.get_mut(&rem).unwrap() -= 1;
            if self.sort.get(&rem).unwrap() == &0 {
                self.sort.remove(&rem);
            }
            self.m_sum -= rem;
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.stream.len() < self.m {
            return -1;
        }

        let mut k_sum = 0;
        let mut k_times = self.k;
        'label: for (num, count) in self.sort.iter().take(self.k) {
            for _ in 0..*count {
                k_sum += num;
                k_times -= 1;
                if k_times < 1 {
                    break 'label;
                }
            }
        }

        k_times = self.k;
        'label2: for (num, count) in self.sort.iter().rev().take(self.k) {
            for _ in 0..*count {
                k_sum += num;
                k_times -= 1;
                if k_times < 1 {
                    break 'label2;
                }
            }
        }

        let sum = self.m_sum - k_sum;
        sum / (self.m - 2 * self.k) as i32
    }
}

#[test]
fn test() {
    let mut obj = MKAverage::new(3, 1);
    obj.add_element(3);
    obj.add_element(1);
    assert_eq!(obj.calculate_mk_average(), -1);
    obj.add_element(10);
    assert_eq!(obj.calculate_mk_average(), 3);
    obj.add_element(5);
    obj.add_element(5);
    obj.add_element(5);
    assert_eq!(obj.calculate_mk_average(), 5);
}
