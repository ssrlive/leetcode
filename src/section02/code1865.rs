#![allow(dead_code)]

/*

// 1865. Finding Pairs With a Certain Sum
// https://leetcode.com/problems/finding-pairs-with-a-certain-sum/
// https://leetcode.cn/problems/finding-pairs-with-a-certain-sum/
//
// Medium
//
// You are given two integer arrays nums1 and nums2. You are tasked to implement a data structure that supports queries of two types:

    Add a positive integer to an element of a given index in the array nums2.
    Count the number of pairs (i, j) such that nums1[i] + nums2[j] equals a given value (0 <= i < nums1.length and 0 <= j < nums2.length).

Implement the FindSumPairs class:

    FindSumPairs(int[] nums1, int[] nums2) Initializes the FindSumPairs object with two integer arrays nums1 and nums2.
    void add(int index, int val) Adds val to nums2[index], i.e., apply nums2[index] += val.
    int count(int tot) Returns the number of pairs (i, j) such that nums1[i] + nums2[j] == tot.

Example 1:

Input
["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"]
[[[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]], [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]
Output
[null, 8, null, 2, 1, null, null, 11]

Explanation
FindSumPairs findSumPairs = new FindSumPairs([1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]);
findSumPairs.count(7);  // return 8; pairs (2,2), (3,2), (4,2), (2,4), (3,4), (4,4) make 2 + 5 and pairs (5,1), (5,5) make 3 + 4
findSumPairs.add(3, 2); // now nums2 = [1,4,5,4,5,4]
findSumPairs.count(8);  // return 2; pairs (5,2), (5,4) make 3 + 5
findSumPairs.count(4);  // return 1; pair (5,0) makes 3 + 1
findSumPairs.add(0, 1); // now nums2 = [2,4,5,4,5,4]
findSumPairs.add(1, 1); // now nums2 = [2,5,5,4,5,4]
findSumPairs.count(7);  // return 11; pairs (2,1), (2,2), (2,4), (3,1), (3,2), (3,4), (4,1), (4,2), (4,4) make 2 + 5 and pairs (5,3), (5,5) make 3 + 4

Constraints:

    1 <= nums1.length <= 1000
    1 <= nums2.length <= 10^5
    1 <= nums1[i] <= 10^9
    1 <= nums2[i] <= 10^5
    0 <= index < nums2.length
    1 <= val <= 10^5
    1 <= tot <= 10^9
    At most 1000 calls are made to add and count each.
*/

use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    sum2: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut sum2 = HashMap::new();
        for &num in &nums2 {
            *sum2.entry(num).or_insert(0) += 1;
        }
        Self { nums1, nums2, sum2 }
    }

    fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let old = self.nums2[index];
        let new = old + val;
        self.nums2[index] = new;
        *self.sum2.entry(old).or_insert(0) -= 1;
        *self.sum2.entry(new).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut ans = 0;
        for &num in &self.nums1 {
            if let Some(&cnt) = self.sum2.get(&(tot - num)) {
                ans += cnt;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let mut find_sum_pairs = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
    assert_eq!(find_sum_pairs.count(7), 8);
    find_sum_pairs.add(3, 2);
    assert_eq!(find_sum_pairs.count(8), 2);
    assert_eq!(find_sum_pairs.count(4), 1);
    find_sum_pairs.add(0, 1);
    find_sum_pairs.add(1, 1);
    assert_eq!(find_sum_pairs.count(7), 11);
}
