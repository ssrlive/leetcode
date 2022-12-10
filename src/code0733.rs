#![allow(dead_code)]

// 733. Flood Fill
// https://leetcode.com/problems/flood-fill/
//
// An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.
//
// You are also given three integers sr, sc, and color. You should perform a flood fill on the image starting from the pixel image[sr][sc].
//
// To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color
// as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on.
// Replace the color of all of the aforementioned pixels with color.
//
// Return the modified image after performing the flood fill.
//
// Example 1:
//
// Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
// Output: [[2,2,2],[2,2,0],[2,0,1]]
// Explanation: From the center of the image with position (sr, sc) = (1, 1) (i.e., the red pixel), all pixels connected by a path
//              of the same color as the starting pixel (i.e., the blue pixels) are colored with the new color.
// Note the bottom corner is not colored 2, because it is not 4-directionally connected to the starting pixel.
//
// Example 2:
//
// Input: image = [[0,0,0],[0,0,0]], sr = 0, sc = 0, color = 0
// Output: [[0,0,0],[0,0,0]]
// Explanation: The starting pixel is already colored 0, so no changes are made to the image.
//
// Constraints:
//
// - m == image.length
// - n == image[i].length
// - 1 <= m, n <= 50
// - 0 <= image[i][j], color < 216
// - 0 <= sr < m
// - 0 <= sc < n
//

struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let (m, n) = (image.len(), image[0].len());
        let (sr, sc) = (sr as usize, sc as usize);
        let old_color = image[sr][sc];
        if old_color == color {
            return image;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((sr, sc));
        while let Some((r, c)) = queue.pop_front() {
            image[r][c] = color;
            if r > 0 && image[r - 1][c] == old_color {
                queue.push_back((r - 1, c));
            }
            if r + 1 < m && image[r + 1][c] == old_color {
                queue.push_back((r + 1, c));
            }
            if c > 0 && image[r][c - 1] == old_color {
                queue.push_back((r, c - 1));
            }
            if c + 1 < n && image[r][c + 1] == old_color {
                queue.push_back((r, c + 1));
            }
        }
        image
    }
}

#[test]
fn test() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let sr = 1;
    let sc = 1;
    let new_color = 2;
    let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    assert_eq!(Solution::flood_fill(image, sr, sc, new_color), expected);
}
