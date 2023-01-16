#![allow(dead_code)]

// 835. Image Overlap
// https://leetcode.com/problems/image-overlap/
// https://leetcode.cn/problems/image-overlap/
//
// You are given two images, img1 and img2, represented as binary, square matrices of size n x n. A binary matrix has only 0s and 1s as values.
//
// We translate one image however we choose by sliding all the 1 bits left, right, up, and/or down any number of units. We then place
// it on top of the other image. We can then calculate the overlap by counting the number of positions that have a 1 in both images.
//
// Note also that a translation does not include any kind of rotation. Any 1 bits that are translated outside of the matrix borders are erased.
//
// Return the largest possible overlap.
//
// Example 1:
//
// Input: img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
// Output: 3
// Explanation: We translate img1 to right by 1 unit and down by 1 unit.
//
// The number of positions that have a 1 in both images is 3 (shown in red).
//
// Example 2:
//
// Input: img1 = [[1]], img2 = [[1]]
// Output: 1
// Example 3:
//
// Input: img1 = [[0]], img2 = [[0]]
// Output: 0
//
// Constraints:
//
// - n == img1.length == img1[i].length
// - n == img2.length == img2[i].length
// - 1 <= n <= 30
// - img1[i][j] is either 0 or 1.
// - img2[i][j] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let n = img1.len() as i32;
        for x_offset in -29..=29i32 {
            for y_offset in -29..=29i32 {
                let mut count = 0;
                for i in 0.max(x_offset)..n.min(n + x_offset) {
                    for j in 0.max(y_offset)..n.min(n + y_offset) {
                        if img1[i as usize][j as usize] == 1 && {
                            let (i_, j_) = ((i - x_offset) as usize, (j - y_offset) as usize);
                            img2[i_][j_] == 1
                        } {
                            count += 1;
                        }
                    }
                }
                max = max.max(count);
            }
        }
        max
    }
}

#[test]
fn test() {
    let img1 = vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
    let img2 = vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]];
    assert_eq!(Solution::largest_overlap(img1, img2), 3);

    let img1 = vec![vec![1]];
    let img2 = vec![vec![1]];
    assert_eq!(Solution::largest_overlap(img1, img2), 1);

    let img1 = vec![vec![0]];
    let img2 = vec![vec![0]];
    assert_eq!(Solution::largest_overlap(img1, img2), 0);

    let img1 = vec![
        vec![0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    let img2 = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::largest_overlap(img1, img2), 1);
}
