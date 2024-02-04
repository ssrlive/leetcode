#![allow(dead_code)]

// 3030. Find the Grid of Region Average
// https://leetcode.com/problems/find-the-grid-of-region-average/
// https://leetcode.cn/problems/find-the-grid-of-region-average/
//
// Medium
//
// You are given a 0-indexed m x n grid image which represents a grayscale image, where image[i][j] represents
// a pixel with intensity in the range[0..255]. You are also given a non-negative integer threshold.
//
// Two pixels image[a][b] and image[c][d] are said to be adjacent if |a - c| + |b - d| == 1.
//
// A region is a 3 x 3 subgrid where the absolute difference in intensity between any two adjacent pixels is less than or equal to threshold.
//
// All pixels in a region belong to that region, note that a pixel can belong to multiple regions.
//
// You need to calculate a 0-indexed m x n grid result, where result[i][j] is the average intensity of the region to
// which image[i][j] belongs, rounded down to the nearest integer. If image[i][j] belongs to multiple regions,
// result[i][j] is the average of the rounded down average intensities of these regions, rounded down to the nearest integer.
// If image[i][j] does not belong to any region, result[i][j] is equal to image[i][j].
//
// Return the grid result.
//
// Example 1:
//
// Input: image = [[5,6,7,10],[8,9,10,10],[11,12,13,10]], threshold = 3
// Output: [[9,9,9,9],[9,9,9,9],[9,9,9,9]]
// Explanation: There exist two regions in the image, which are shown as the shaded areas in the picture.
// The average intensity of the first region is 9, while the average intensity of the second region is 9.67 which is rounded down to 9.
// The average intensity of both of the regions is (9 + 9) / 2 = 9. As all the pixels belong to either region 1, region 2, or both of them,
// the intensity of every pixel in the result is 9.
// Please note that the rounded-down values are used when calculating the average of multiple regions,
// hence the calculation is done using 9 as the average intensity of region 2, not 9.67.
//
// Example 2:
//
// Input: image = [[10,20,30],[15,25,35],[20,30,40],[25,35,45]], threshold = 12
// Output: [[25,25,25],[27,27,27],[27,27,27],[30,30,30]]
// Explanation: There exist two regions in the image, which are shown as the shaded areas in the picture.
// The average intensity of the first region is 25, while the average intensity of the second region is 30.
// The average intensity of both of the regions is (25 + 30) / 2 = 27.5 which is rounded down to 27.
// All the pixels in row 0 of the image belong to region 1, hence all the pixels in row 0 in the result are 25. Similarly,
// all the pixels in row 3 in the result are 30. The pixels in rows 1 and 2 of the image belong
// to region 1 and region 2, hence their assigned value is 27 in the result.
//
// Example 3:
//
// Input: image = [[5,6,7],[8,9,10],[11,12,13]], threshold = 1
// Output: [[5,6,7],[8,9,10],[11,12,13]]
// Explanation: There does not exist any region in image, hence result[i][j] == image[i][j] for all the pixels.
//
// Constraints:
//
// 3 <= n, m <= 500
// 0 <= image[i][j] <= 255
// 0 <= threshold <= 255
//

struct Solution;

impl Solution {
    pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
        let original_image = image.clone();
        let (n, m) = (image.len(), image[0].len());
        let mut result = original_image.clone();
        let mut regions = vec![vec![vec![]; m]; n];

        fn calculate_metrics(image: &[Vec<i32>], row: usize, col: usize) -> (i32, i32) {
            let (mut max_diff, mut sum_intensity) = (0, 0);
            for i in row..=row + 2 {
                for j in col..=col + 2 {
                    if i != row + 2 {
                        max_diff = max_diff.max((image[i][j] - image[i + 1][j]).abs());
                    }
                    if j != col + 2 {
                        max_diff = max_diff.max((image[i][j] - image[i][j + 1]).abs());
                    }
                    sum_intensity += image[i][j];
                }
            }
            (max_diff, sum_intensity)
        }

        for row in 0..n - 2 {
            for col in 0..m - 2 {
                let (max_diff, sum_intensity) = calculate_metrics(&original_image, row, col);
                if max_diff <= threshold {
                    for regions_i in regions.iter_mut().skip(row).take(2 + 1) {
                        for regions_i_j in regions_i.iter_mut().skip(col).take(2 + 1) {
                            regions_i_j.push(sum_intensity / 9);
                        }
                    }
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if !regions[i][j].is_empty() {
                    let mut sum = 0;
                    for intensity in &regions[i][j] {
                        sum += intensity;
                    }
                    result[i][j] = sum / regions[i][j].len() as i32;
                }
            }
        }

        result
    }
}

#[test]
fn test() {
    let image = vec![vec![5, 6, 7, 10], vec![8, 9, 10, 10], vec![11, 12, 13, 10]];
    let threshold = 3;
    let result = vec![vec![9, 9, 9, 9], vec![9, 9, 9, 9], vec![9, 9, 9, 9]];
    assert_eq!(Solution::result_grid(image, threshold), result);

    let image = vec![vec![10, 20, 30], vec![15, 25, 35], vec![20, 30, 40], vec![25, 35, 45]];
    let threshold = 12;
    let result = vec![vec![25, 25, 25], vec![27, 27, 27], vec![27, 27, 27], vec![30, 30, 30]];
    assert_eq!(Solution::result_grid(image, threshold), result);

    let image = vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]];
    let threshold = 1;
    let result = vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]];
    assert_eq!(Solution::result_grid(image, threshold), result);
}
