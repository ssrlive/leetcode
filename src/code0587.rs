#![allow(dead_code)]

// 587. Erect the Fence
// https://leetcode.com/problems/erect-the-fence/
//
// You are given an array trees where trees[i] = [xi, yi] represents the location of a tree in the garden.
//
// Fence the entire garden using the minimum length of rope, as it is expensive.
// The garden is well-fenced only if all the trees are enclosed.
//
// Return the coordinates of trees that are exactly located on the fence perimeter.
// You may return the answer in any order.
//
// Example 1:
//
// Input: trees = [[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]
// Output: [[1,1],[2,0],[4,2],[3,3],[2,4]]
// Explanation: All the trees will be on the perimeter of the fence except the tree at [2, 2], which will be inside the fence.
//
// Example 2:
//
// Input: trees = [[1,2],[2,2],[4,2]]
// Output: [[4,2],[2,2],[1,2]]
// Explanation: The fence forms a line that passes through all the trees.
//
// Constraints:
//
// - 1 <= trees.length <= 3000
// - trees[i].length == 2
// - 0 <= xi, yi <= 100
// - All the given positions are unique.
//

struct Solution;

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees;
        let mut ret = vec![trees.iter().min().unwrap().to_owned()];

        let mut prev_angle = 0.;
        loop {
            let prev = ret.last().unwrap();
            let (min_angle_i, (min_angle, _)) = trees
                .iter()
                .enumerate()
                .filter(|(_, t)| ret.len() > 1 || trees.len() == 1 || *t != prev)
                .map(|(i, t)| {
                    let mut angle = ((t[0] - prev[0]) as f32).atan2((t[1] - prev[1]) as f32);
                    angle += if angle < 0. { 2. * std::f32::consts::PI } else { 0. };
                    let dist: i32 = prev.iter().zip(t.iter()).map(|(a, b)| (b - a).pow(2)).sum();
                    (i, (angle, dist))
                })
                .filter(|(_, (angle, _))| *angle >= prev_angle)
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            if trees[min_angle_i] == ret[0] {
                return ret;
            }
            ret.push(trees.remove(min_angle_i));
            prev_angle = min_angle;
        }
    }
}

#[test]
fn test() {
    let trees = vec![vec![1, 1], vec![2, 2], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]];
    let expected = vec![vec![1, 1], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]];
    let mut res = Solution::outer_trees(trees);
    res.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    assert_eq!(res, expected);

    let trees = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
    let expected = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
    let mut res = Solution::outer_trees(trees);
    res.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    assert_eq!(res, expected);
}
