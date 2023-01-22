#![allow(dead_code)]

// 2545. Sort the Students by Their Kth Score
// https://leetcode.com/problems/sort-the-students-by-their-kth-score/
// https://leetcode.cn/problems/sort-the-students-by-their-kth-score/
//
// Medium
//
// There is a class with m students and n exams. You are given a 0-indexed m x n integer matrix score,
// where each row represents one student and score[i][j] denotes the score the ith student got in the jth exam.
// The matrix score contains distinct integers only.
//
// You are also given an integer k. Sort the students (i.e., the rows of the matrix) by their scores
// in the kth (0-indexed) exam from the highest to the lowest.
//
// Return the matrix after sorting it.
//
// Example 1:
//
// Input: score = [[10,6,9,1],[7,5,11,2],[4,8,3,15]], k = 2
// Output: [[7,5,11,2],[10,6,9,1],[4,8,3,15]]
// Explanation: In the above diagram, S denotes the student, while E denotes the exam.
// - The student with index 1 scored 11 in exam 2, which is the highest score, so they got first place.
// - The student with index 0 scored 9 in exam 2, which is the second highest score, so they got second place.
// - The student with index 2 scored 3 in exam 2, which is the lowest score, so they got third place.
//
// Example 2:
//
// Input: score = [[3,4],[5,6]], k = 0
// Output: [[5,6],[3,4]]
// Explanation: In the above diagram, S denotes the student, while E denotes the exam.
// - The student with index 1 scored 5 in exam 0, which is the highest score, so they got first place.
// - The student with index 0 scored 3 in exam 0, which is the lowest score, so they got second place.
//
// Constraints:
//
// -    m == score.length
// -    n == score[i].length
// -    1 <= m, n <= 250
// -    1 <= score[i][j] <= 10^5
// -    score consists of distinct integers.
// -    0 <= k < n
//

struct Solution;

impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score;
        score.sort_by(|a, b| b[k as usize].cmp(&a[k as usize]));
        score
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
            2,
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]],
        ),
        (vec![vec![3, 4], vec![5, 6]], 0, vec![vec![5, 6], vec![3, 4]]),
    ];
    for (score, k, expected) in cases {
        assert_eq!(Solution::sort_the_students(score, k), expected);
    }
}
