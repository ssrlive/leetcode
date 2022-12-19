#![allow(dead_code)]

// 769. Max Chunks To Make Sorted
// https://leetcode.com/problems/max-chunks-to-make-sorted/
// https://leetcode.cn/problems/max-chunks-to-make-sorted/
//
// You are given an integer array arr of length n that represents a permutation of the integers in the range [0, n - 1].
//
// We split arr into some number of chunks (i.e., partitions), and individually sort each chunk.
// After concatenating them, the result should equal the sorted array.
//
// Return the largest number of chunks we can make to sort the array.
//
// Example 1:
//
// Input: arr = [4,3,2,1,0]
// Output: 1
// Explanation:
// Splitting into two or more chunks will not return the required result.
// For example, splitting into [4, 3], [2, 1, 0] will result in [3, 4, 0, 1, 2], which isn't sorted.
//
// Example 2:
//
// Input: arr = [1,0,2,3,4]
// Output: 4
// Explanation:
// We can split into two chunks, such as [1, 0], [2, 3, 4].
// However, splitting into [1, 0], [2], [3], [4] is the highest number of chunks possible.
//
// Constraints:
//
// - n == arr.length
// - 1 <= n <= 10
// - 0 <= arr[i] < n
// - All the elements of arr are unique.
//

struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut _max = 0;
        let mut chunks = 0;

        for (i, &n) in arr.iter().enumerate() {
            _max = std::cmp::max(_max, n);
            if _max == i as i32 {
                chunks += 1;
            }
        }

        chunks
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
}
