#![allow(dead_code)]

// 768. Max Chunks To Make Sorted II
// https://leetcode.com/problems/max-chunks-to-make-sorted-ii/
// https://leetcode.cn/problems/max-chunks-to-make-sorted-ii/
//
// You are given an integer array arr.
//
// We split arr into some number of chunks (i.e., partitions), and individually sort each chunk.
// After concatenating them, the result should equal the sorted array.
//
// Return the largest number of chunks we can make to sort the array.
//
// Example 1:
//
// Input: arr = [5,4,3,2,1]
// Output: 1
// Explanation:
// Splitting into two or more chunks will not return the required result.
// For example, splitting into [5, 4], [3, 2, 1] will result in [4, 5, 1, 2, 3], which isn't sorted.
//
// Example 2:
//
// Input: arr = [2,1,3,4,4]
// Output: 4
// Explanation:
// We can split into two chunks, such as [2, 1], [3, 4, 4].
// However, splitting into [2, 1], [3], [4], [4] is the highest number of chunks possible.
//
// Constraints:
//
// - 1 <= arr.length <= 2000
// - 0 <= arr[i] <= 10^8
//

struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let arr_float = arr
            .iter()
            .enumerate()
            .map(|(i, &x)| x as f32 + (i as f32 / 10000.0))
            .collect::<Vec<f32>>();

        let mut arr_float_sorted = arr_float.clone();
        arr_float_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut max = 0.0;
        let mut res = 0;

        for (index, num) in arr_float.iter().enumerate() {
            if num > &max {
                max = *num as f32;
            }
            if arr_float_sorted[index] == max {
                res += 1;
                max = 0.0;
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
}
