#![allow(dead_code)]

// 303. Range Sum Query - Immutable
// https://leetcode.com/problems/range-sum-query-immutable/
//
// Given an integer array nums, handle multiple queries of the following type:
//
// Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
//
// Implement the NumArray class:
//
// NumArray(int[] nums) Initializes the object with the integer array nums.
// int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right
// inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).
//
//
// Example 1:
//
// Input
// ["NumArray", "sumRange", "sumRange", "sumRange"]
// [[[1, 2, 3, 4]], [0, 2], [2, 3], [0, 3]]
// Output
// [null, 6, 7, 10]
//
// Explanation
// NumArray numArray = new NumArray([1, 2, 3, 4]);
// numArray.sumRange(0, 2); // return 1 + 2 + 3 = 6
// numArray.sumRange(2, 3); // return 3 + 4 = 7
// numArray.sumRange(0, 3); // return 1 + 2 + 3 + 4 = 10
//
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -10^5 <= nums[i] <= 10^5
// - 0 <= left <= right < nums.length
// - At most 10^4 calls will be made to sumRange.
//

struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sums[i + 1] = sums[i] + nums[i];
        }
        Self { sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[right as usize + 1] - self.sums[left as usize]
    }
}

#[test]
fn test_sum_range() {
    let num_array = NumArray::new(vec![1, 2, 3, 4]);
    assert_eq!(num_array.sum_range(0, 2), 6);
    assert_eq!(num_array.sum_range(2, 3), 7);
    assert_eq!(num_array.sum_range(0, 3), 10);
}
