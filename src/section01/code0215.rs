#![allow(dead_code)]

// 215. Kth Largest Element in an Array
// https://leetcode.com/problems/kth-largest-element-in-an-array/
// https://leetcode.cn/problems/kth-largest-element-in-an-array/
//

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        fn _find_kth_largest(nums: Vec<i32>, k: i32) -> Option<i32> {
            let mut heap = std::collections::BinaryHeap::new();
            for n in nums {
                heap.push(n);
            }
            for _ in 0..k - 1 {
                heap.pop();
            }
            heap.pop()
        }

        _find_kth_largest(nums, k).unwrap_or_default()
    }
}

#[test]
fn test_find_kth_largest() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}
