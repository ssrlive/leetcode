#![allow(dead_code)]

/*

// 1574. Shortest Subarray to be Removed to Make Array Sorted
Medium
1.3K
47
Companies

Given an integer array arr, remove a subarray (can be empty) from arr such that the remaining elements in arr are non-decreasing.

Return the length of the shortest subarray to remove.

A subarray is a contiguous subsequence of the array.

Example 1:

Input: arr = [1,2,3,10,4,2,3,5]
Output: 3
Explanation: The shortest subarray we can remove is [10,4,2] of length 3. The remaining elements after that will be [1,2,3,3,5] which are sorted.
Another correct solution is to remove the subarray [3,10,4].

Example 2:

Input: arr = [5,4,3,2,1]
Output: 4
Explanation: Since the array is strictly decreasing, we can only keep a single element. Therefore we need to remove a subarray of length 4, either [5,4,3,2] or [4,3,2,1].

Example 3:

Input: arr = [1,2,3]
Output: 0
Explanation: The array is already non-decreasing. We do not need to remove any elements.

Constraints:

    1 <= arr.length <= 10^5
    0 <= arr[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = 0;
        while left + 1 < n && arr[left] <= arr[left + 1] {
            left += 1;
        }
        if left == n - 1 {
            return 0;
        }
        let mut right = n - 1;
        while right > 0 && arr[right - 1] <= arr[right] {
            right -= 1;
        }
        let mut res = (n - left - 1).min(right) as i32;
        let mut i = 0;
        let mut j = right;
        while i <= left && j < n {
            if arr[i] <= arr[j] {
                res = res.min((j - i - 1) as i32);
                i += 1;
            } else {
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3, 10, 4, 2, 3, 5], 3), (vec![5, 4, 3, 2, 1], 4), (vec![1, 2, 3], 0)];
    for (arr, expected) in cases {
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), expected);
    }
}
