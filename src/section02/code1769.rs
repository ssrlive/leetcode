#![allow(dead_code)]

/*

// 1769. Minimum Number of Operations to Move All Balls to Each Box
Medium
1.9K
70
Companies

You have n boxes. You are given a binary string boxes of length n, where boxes[i] is '0' if the ith box is empty, and '1' if it contains one ball.

In one operation, you can move one ball from a box to an adjacent box. Box i is adjacent to box j if abs(i - j) == 1. Note that after doing so, there may be more than one ball in some boxes.

Return an array answer of size n, where answer[i] is the minimum number of operations needed to move all the balls to the ith box.

Each answer[i] is calculated considering the initial state of the boxes.

Example 1:

Input: boxes = "110"
Output: [1,1,3]
Explanation: The answer for each box is as follows:
1) First box: you will have to move one ball from the second box to the first box in one operation.
2) Second box: you will have to move one ball from the first box to the second box in one operation.
3) Third box: you will have to move one ball from the first box to the third box in two operations, and move one ball from the second box to the third box in one operation.

Example 2:

Input: boxes = "001011"
Output: [11,8,5,4,3,4]

Constraints:

    n == boxes.length
    1 <= n <= 2000
    boxes[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let boxes = boxes.chars().collect::<Vec<char>>();
        let (mut left_cnt, mut right_cnt) = (vec![0; n], vec![0; n]);
        let (mut left_from, mut right_from) = (vec![0; n], vec![0; n]);
        let mut ret = vec![0; n];
        for i in 0..n {
            if boxes[i] == '1' {
                left_cnt[i] += 1;
            }
            if i == 0 {
                continue;
            }
            left_cnt[i] += left_cnt[i - 1];
            left_from[i] = left_cnt[i - 1] + left_from[i - 1];
            ret[i] += left_from[i];
        }
        for i in (0..n).rev() {
            if boxes[i] == '1' {
                right_cnt[i] += 1;
            }
            if i == n - 1 {
                continue;
            }
            right_cnt[i] += right_cnt[i + 1];
            right_from[i] = right_cnt[i + 1] + right_from[i + 1];
            ret[i] += right_from[i];
        }
        ret
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("110", vec![1, 1, 3]),
        ("001011", vec![11, 8, 5, 4, 3, 4]),
    ];
    for (boxes, expected) in cases {
        assert_eq!(Solution::min_operations(boxes.to_string()), expected);
    }
}
