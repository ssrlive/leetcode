#![allow(dead_code)]

/*

// 1775. Equal Sum Arrays With Minimum Number of Operations
Medium
780
27
Companies

You are given two arrays of integers nums1 and nums2, possibly of different lengths. The values in the arrays are between 1 and 6, inclusive.

In one operation, you can change any integer's value in any of the arrays to any value between 1 and 6, inclusive.

Return the minimum number of operations required to make the sum of values in nums1 equal to the sum of values in nums2. Return -1​​​​​ if it is not possible to make the sum of the two arrays equal.

Example 1:

Input: nums1 = [1,2,3,4,5,6], nums2 = [1,1,2,2,2,2]
Output: 3
Explanation: You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums2[0] to 6. nums1 = [1,2,3,4,5,6], nums2 = [6,1,2,2,2,2].
- Change nums1[5] to 1. nums1 = [1,2,3,4,5,1], nums2 = [6,1,2,2,2,2].
- Change nums1[2] to 2. nums1 = [1,2,2,4,5,1], nums2 = [6,1,2,2,2,2].

Example 2:

Input: nums1 = [1,1,1,1,1,1,1], nums2 = [6]
Output: -1
Explanation: There is no way to decrease the sum of nums1 or to increase the sum of nums2 to make them equal.

Example 3:

Input: nums1 = [6,6], nums2 = [1]
Output: 3
Explanation: You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums1[0] to 2. nums1 = [2,6], nums2 = [1].
- Change nums1[1] to 2. nums1 = [2,2], nums2 = [1].
- Change nums2[0] to 4. nums1 = [2,2], nums2 = [4].

Constraints:

    1 <= nums1.length, nums2.length <= 10^5
    1 <= nums1[i], nums2[i] <= 6
*/

struct Solution;

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let a: i32 = nums1.iter().copied().sum();
        let b = nums2.iter().copied().sum();
        if a < b {
            return Self::min_operations(nums2, nums1);
        }
        let mut pq = BinaryHeap::new();
        let mut amt = a - b;
        for n in nums1 {
            if n > 1 {
                pq.push(n - 1);
            }
        }
        for n in nums2 {
            if n < 6 {
                pq.push(6 - n);
            }
        }
        let mut ret = 0;
        while amt > 0 && !pq.is_empty() {
            amt -= pq.pop().unwrap();
            ret += 1;
        }
        if amt > 0 {
            return -1;
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2], 3),
        (vec![1, 1, 1, 1, 1, 1, 1], vec![6], -1),
        (vec![6, 6], vec![1], 3),
    ];
    for (nums1, nums2, expected) in cases {
        assert_eq!(Solution::min_operations(nums1, nums2), expected);
    }
}
