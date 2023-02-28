#![allow(dead_code)]

/*

// 1901. Find a Peak Element II
// https://leetcode.com/problems/find-a-peak-element-ii/
// https://leetcode.cn/problems/find-a-peak-element-ii/
//
// Medium
//
// A peak element in a 2D grid is an element that is strictly greater than all of its adjacent neighbors to the left, right, top, and bottom.

Given a 0-indexed m x n matrix mat where no two adjacent cells are equal, find any peak element mat[i][j] and return the length 2 array [i,j].

You may assume that the entire matrix is surrounded by an outer perimeter with the value -1 in each cell.

You must write an algorithm that runs in O(m log(n)) or O(n log(m)) time.

Example 1:

Input: mat = [[1,4],[3,2]]
Output: [0,1]
Explanation: Both 3 and 4 are peak elements so [1,0] and [0,1] are both acceptable answers.

Example 2:

Input: mat = [[10,20,15],[21,30,14],[7,16,32]]
Output: [1,1]
Explanation: Both 30 and 32 are peak elements so [1,1] and [2,2] are both acceptable answers.

Constraints:

    m == mat.length
    n == mat[i].length
    1 <= m, n <= 500
    1 <= mat[i][j] <= 10^5
    No two adjacent cells are equal.
*/

struct Solution;

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        fn get_max_index(a: &[i32]) -> usize {
            let mut ind = 0;
            let mut mx = 0;
            for (j, &v) in a.iter().enumerate() {
                if v > mx {
                    mx = v;
                    ind = j;
                }
            }
            ind
        }

        let n = mat.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut ind = 0;
        while l < r {
            let mid = (l + r) / 2;
            ind = get_max_index(&mat[mid]);
            if mid + 1 >= n || mat[mid + 1][ind] < mat[mid][ind] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        vec![l as i32, ind as i32]
    }
}

#[test]
fn test() {
    let mat = vec![vec![1, 4], vec![3, 2]];
    let result = Solution::find_peak_grid(mat);
    assert_eq!(result, vec![0, 1]);
    let mat = vec![vec![10, 20, 15], vec![21, 30, 14], vec![7, 16, 32]];
    let result = Solution::find_peak_grid(mat);
    assert_eq!(result, vec![1, 1]);
}
