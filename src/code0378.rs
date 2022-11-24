#![allow(dead_code)]

// 378. Kth Smallest Element in a Sorted Matrix
// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
//
// Given an n x n matrix where each of the rows and columns is sorted in ascending order, return the kth smallest element in the matrix.
//
// Note that it is the kth smallest element in the sorted order, not the kth distinct element.
//
// You must find a solution with a memory complexity better than O(n2).
//
// Example 1:
//
// Input: matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
// Output: 13
// Explanation: The elements in the matrix are [1,5,9,10,11,12,13,13,15], and the 8th smallest number is 13
//
// Example 2:
//
// Input: matrix = [[-5]], k = 1
// Output: -5
//
// Constraints:
//
// - n == matrix.length == matrix[i].length
// - 1 <= n <= 300
// - -109 <= matrix[i][j] <= 109
// - All the rows and columns of matrix are guaranteed to be sorted in non-decreasing order.
// - 1 <= k <= n2
//
// Follow up:
//
// Could you solve the problem with a constant memory (i.e., O(1) memory complexity)?
// Could you solve the problem in O(n) time complexity? The solution may be too advanced for an interview but you may find reading this paper fun.
//

struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        for row in matrix.iter() {
            for col in row.iter() {
                heap.push(std::cmp::Reverse(col));
            }
        }
        let mut ans = Some(std::cmp::Reverse(&-1));
        for _ in 0..k {
            ans = heap.pop();
        }
        match ans {
            Some(std::cmp::Reverse(num)) => *num,
            _ => -1,
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
        13
    );
    assert_eq!(Solution::kth_smallest(vec![vec![-5]], 1), -5);
}
