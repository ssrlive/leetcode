#![allow(dead_code)]

/*

// 2101. Detonate the Maximum Bombs
// https://leetcode.com/problems/detonate-the-maximum-bombs/
// https://leetcode.cn/problems/detonate-the-maximum-bombs/
//
// Medium
//
// You are given a list of bombs. The range of a bomb is defined as the area where its effect can be felt. This area is in the shape of a circle with the center as the location of the bomb.

The bombs are represented by a 0-indexed 2D integer array bombs where bombs[i] = [xi, yi, ri]. xi and yi denote the X-coordinate and Y-coordinate of the location of the ith bomb, whereas ri denotes the radius of its range.

You may choose to detonate a single bomb. When a bomb is detonated, it will detonate all bombs that lie in its range. These bombs will further detonate the bombs that lie in their ranges.

Given the list of bombs, return the maximum number of bombs that can be detonated if you are allowed to detonate only one bomb.

Example 1:

Input: bombs = [[2,1,3],[6,1,4]]
Output: 2
Explanation:
The above figure shows the positions and ranges of the 2 bombs.
If we detonate the left bomb, the right bomb will not be affected.
But if we detonate the right bomb, both bombs will be detonated.
So the maximum bombs that can be detonated is max(1, 2) = 2.

Example 2:

Input: bombs = [[1,1,5],[10,10,5]]
Output: 1
Explanation:
Detonating either bomb will not detonate the other bomb, so the maximum number of bombs that can be detonated is 1.

Example 3:

Input: bombs = [[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]]
Output: 5
Explanation:
The best bomb to detonate is bomb 0 because:
- Bomb 0 detonates bombs 1 and 2. The red circle denotes the range of bomb 0.
- Bomb 2 detonates bomb 3. The blue circle denotes the range of bomb 2.
- Bomb 3 detonates bomb 4. The green circle denotes the range of bomb 3.
Thus all 5 bombs are detonated.

Constraints:

    1 <= bombs.length <= 100
    bombs[i].length == 3
    1 <= xi, yi, ri <= 10^5
*/

struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        use std::collections::*;

        fn dfs(list: &Vec<Vec<usize>>, current: usize, visited: &mut HashSet<usize>, all_visited: &mut HashSet<usize>) -> usize {
            visited.insert(current);
            let mut res = 1; // Detonates self

            for i in &list[current] {
                if visited.contains(i) {
                    continue;
                }
                visited.insert(*i);
                all_visited.insert(*i);
                res += dfs(list, *i, visited, all_visited);
            }
            res
        }

        let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; bombs.len()];

        for i in 0..bombs.len() {
            for j in i + 1..bombs.len() {
                let xi = bombs[i][0];
                let yi = bombs[i][1];
                let ri = bombs[i][2];

                let xj = bombs[j][0];
                let yj = bombs[j][1];
                let rj = bombs[j][2];

                let d2 = ((xi as f32 - xj as f32).powi(2) + (yi as f32 - yj as f32).powi(2)).round() as i64;
                if d2 <= (ri as i64).pow(2) {
                    adjacency_list[i].push(j);
                }
                if d2 <= (rj as i64).pow(2) {
                    adjacency_list[j].push(i);
                }
            }
        }

        let mut max = 0;
        let mut all_visited = HashSet::new();
        for i in 0..bombs.len() {
            if all_visited.contains(&i) {
                continue;
            }
            let val = dfs(&adjacency_list, i, &mut HashSet::new(), &mut all_visited);
            max = max.max(val);
        }

        max as i32
    }
}

#[test]
fn test() {}
