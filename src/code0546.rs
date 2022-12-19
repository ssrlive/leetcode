#![allow(dead_code)]

// 546. Remove Boxes
// https://leetcode.com/problems/remove-boxes/
// https://leetcode.cn/problems/remove-boxes/
//
// You are given several boxes with different colors represented by different positive numbers.
//
// You may experience several rounds to remove boxes until there is no box left. Each time you can choose some continuous boxes with the same color (i.e., composed of k boxes, k >= 1), remove them and get k * k points.
//
// Return the maximum points you can get.
//
// Example 1:
//
// Input: boxes = [1,3,2,2,2,3,4,3,1]
// Output: 23
// Explanation:
// [1, 3, 2, 2, 2, 3, 4, 3, 1]
// ----> [1, 3, 3, 4, 3, 1] (3*3=9 points)
// ----> [1, 3, 3, 3, 1] (1*1=1 points)
// ----> [1, 1] (3*3=9 points)
// ----> [] (2*2=4 points)
//
// Example 2:
//
// Input: boxes = [1,1,1]
// Output: 9
//
// Example 3:
//
// Input: boxes = [1]
// Output: 1
//
// Constraints:
//
// - 1 <= boxes.length <= 100
// - 1 <= boxes[i] <= 100
//

struct Solution;

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let len = boxes.len();
        let mut memo = vec![vec![vec![None; len]; len]; len];
        Self::dp(&boxes, &mut memo, 0, len - 1, 0)
    }
    fn dp(boxes: &[i32], memo: &mut Vec<Vec<Vec<Option<i32>>>>, i: usize, j: usize, k: usize) -> i32 {
        if let Some(val) = memo[i][j][k] {
            return val;
        }
        let mut mi = i;
        let mut mk = k as i32;
        while mi < j && boxes[mi + 1] == boxes[mi] {
            mi += 1;
            mk += 1;
        }
        let mut max = (mk + 1) * (mk + 1) + if mi < j { Self::dp(boxes, memo, mi + 1, j, 0) } else { 0 };
        for l in mi + 1..=j {
            if boxes[l] == boxes[i] {
                let val1 = Self::dp(boxes, memo, mi + 1, l - 1, 0);
                let val2 = Self::dp(boxes, memo, l, j, mk as usize + 1);
                max = max.max(val1 + val2);
            }
        }
        memo[i][j][k] = Some(max);
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
    assert_eq!(Solution::remove_boxes(vec![1, 1, 1]), 9);
    assert_eq!(Solution::remove_boxes(vec![1]), 1);

    let boxes = vec![
        1, 2, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1, 2, 1, 1, 2, 2, 1, 2, 1, 2,
        2, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 2, 2, 1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 1,
        1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1,
    ];
    assert_eq!(Solution::remove_boxes(boxes), 2758);
}
