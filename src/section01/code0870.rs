#![allow(dead_code)]

// 870. Advantage Shuffle
// https://leetcode.com/problems/advantage-shuffle/
// https://leetcode.cn/problems/advantage-shuffle/
//
// You are given two integer arrays nums1 and nums2 both of the same length. The advantage of nums1 with respect to nums2 is the number of indices i for which nums1[i] > nums2[i].
//
// Return any permutation of nums1 that maximizes its advantage with respect to nums2.
//
// Example 1:
//
// Input: nums1 = [2,7,11,15], nums2 = [1,10,4,11]
// Output: [2,11,7,15]
//
// Example 2:
//
// Input: nums1 = [12,24,8,32], nums2 = [13,25,32,11]
// Output: [24,32,8,12]
//
// Constraints:
//
// - 1 <= nums1.length <= 10^5
// - nums2.length == nums1.length
// - 0 <= nums1[i], nums2[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut a_sorted = nums1;
        let mut b_sorted = nums2.clone();
        a_sorted.sort_unstable();
        b_sorted.sort_unstable();
        let mut hm: HashMap<_, Vec<_>> = HashMap::new();
        let mut remain = Vec::new();
        let mut i = 0;
        for &n in &a_sorted {
            if n > b_sorted[i] {
                hm.entry(b_sorted[i]).or_default().push(n);
                i += 1;
            } else {
                remain.push(n);
            }
        }
        nums2
            .into_iter()
            .filter_map(|n| hm.get_mut(&n).filter(|v| !v.is_empty()).map_or_else(|| remain.pop(), |v| v.pop()))
            .collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 7, 11, 15], vec![1, 10, 4, 11], vec![2, 11, 7, 15]),
        (vec![12, 24, 8, 32], vec![13, 25, 32, 11], vec![24, 32, 8, 12]),
    ];
    for (nums1, nums2, expected) in cases {
        assert_eq!(Solution::advantage_count(nums1, nums2), expected);
    }
}
