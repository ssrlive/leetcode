#![allow(dead_code)]

// 1024. Video Stitching
// https://leetcode.com/problems/video-stitching/
// https://leetcode.cn/problems/video-stitching/
//
// You are given a series of video clips from a sporting event that lasted time seconds. These video clips can be overlapping with each other and have varying lengths.
//
// Each video clip is described by an array clips where clips[i] = [starti, endi] indicates that the ith clip started at starti and ended at endi.
//
// We can cut these clips into segments freely.
//
// For example, a clip [0, 7] can be cut into segments [0, 1] + [1, 3] + [3, 7].
// Return the minimum number of clips needed so that we can cut the clips into segments that cover the entire sporting event [0, time]. If the task is impossible, return -1.
//
// Example 1:
//
// Input: clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]], time = 10
// Output: 3
// Explanation: We take the clips [0,2], [8,10], [1,9]; a total of 3 clips.
// Then, we can reconstruct the sporting event as follows:
// We cut [1,9] into segments [1,2] + [2,8] + [8,9].
// Now we have segments [0,2] + [2,8] + [8,10] which cover the sporting event [0, 10].
//
// Example 2:
//
// Input: clips = [[0,1],[1,2]], time = 5
// Output: -1
// Explanation: We cannot cover [0,5] with only [0,1] and [1,2].
//
// Example 3:
//
// Input: clips = [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]], time = 9
// Output: 3
// Explanation: We can take clips [0,4], [4,7], and [6,9].
//
// Constraints:
//
// - 1 <= clips.length <= 100
// - 0 <= starti <= endi <= 100
// - 1 <= time <= 100
//

struct Solution;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let inf = 10000;

        let n = time as usize;
        let mut g = vec![vec![]; 100 + 1];

        for arr in clips {
            let a = arr[0] as usize;
            let b = arr[1] as usize;
            for i in a..b {
                for j in i + 1..=b {
                    g[a].push(j);
                }
            }
        }

        let mut memo = vec![inf; 100 + 1];
        let mut stack = vec![(0, 0)];
        memo[0] = 0;
        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some((ci, cv)) = stack.pop() {
                let nv = cv + 1;
                for &ni in &g[ci] {
                    if nv < memo[ni] {
                        memo[ni] = nv;
                        new_stack.push((ni, nv));
                    }
                }
            }
            stack = new_stack;
        }

        if memo[n] == 10000 {
            -1
        } else {
            memo[n]
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![1, 9], vec![1, 5], vec![5, 9]],
            10,
            3,
        ),
        (vec![vec![0, 1], vec![1, 2]], 5, -1),
        (
            vec![
                vec![0, 1],
                vec![6, 8],
                vec![0, 2],
                vec![5, 6],
                vec![0, 4],
                vec![0, 3],
                vec![6, 7],
                vec![1, 3],
                vec![4, 7],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![4, 5],
                vec![5, 7],
                vec![6, 9],
            ],
            9,
            3,
        ),
    ];
    for (clips, time, expected) in cases {
        assert_eq!(Solution::video_stitching(clips, time), expected);
    }
}
