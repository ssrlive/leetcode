#![allow(dead_code)]

// 3132. Find the Integer Added to Array II
// https://leetcode.com/problems/find-the-integer-added-to-array-ii/
// https://leetcode.cn/problems/find-the-integer-added-to-array-ii/
//
// Medium
//
// You are given two integer arrays nums1 and nums2.
//
// From nums1 two elements have been removed, and all other elements have been increased (or decreased in the case of negative)
// by an integer, represented by the variable x.
//
// As a result, nums1 becomes equal to nums2. Two arrays are considered equal when they contain the same integers with the same frequencies.
//
// Return the minimum possible integer x that achieves this equivalence.
//
// Example 1:
//
// Input: nums1 = [4,20,16,12,8], nums2 = [14,18,10]
//
// Output: -2
//
// Explanation:
//
// After removing elements at indices [0,4] and adding -2, nums1 becomes [18,14,10].
//
// Example 2:
//
// Input: nums1 = [3,5,5,3], nums2 = [7,7]
//
// Output: 2
//
// Explanation:
//
// After removing elements at indices [0,3] and adding 2, nums1 becomes [7,7].
//
// Constraints:
//
// 3 <= nums1.length <= 200
// nums2.length == nums1.length - 2
// 0 <= nums1[i], nums2[i] <= 1000
// The test cases are generated in a way that there is an integer x such that nums1 can become equal to nums2
// by removing two elements and adding x to each element of nums1.
//

struct Solution;

impl Solution {
    pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut ans = std::i32::MAX;

        for i in 0..nums1.len() {
            let x = nums2[0] - nums1[i];
            if Self::check(&nums1, &nums2, x) {
                ans = ans.min(x);
            }
        }

        ans
    }

    fn check(nums1: &[i32], nums2: &[i32], x: i32) -> bool {
        let mut count = 0;
        let mut j = 0;

        for &nums1_i in nums1 {
            if j < nums2.len() {
                if nums1_i + x != nums2[j] {
                    count += 1;
                } else {
                    j += 1;
                }
            } else {
                break;
            }
        }

        count <= 2
    }
}

#[test]
fn test() {
    let nums1 = vec![4, 20, 16, 12, 8];
    let nums2 = vec![14, 18, 10];
    let ans = -2;
    assert_eq!(Solution::minimum_added_integer(nums1, nums2), ans);

    let nums1 = vec![3, 5, 5, 3];
    let nums2 = vec![7, 7];
    let ans = 2;
    assert_eq!(Solution::minimum_added_integer(nums1, nums2), ans);
}
