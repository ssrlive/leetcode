#![allow(dead_code)]

/*

// 1889. Minimum Space Wasted From Packaging
// https://leetcode.com/problems/minimum-space-wasted-from-packaging/
// https://leetcode.cn/problems/minimum-space-wasted-from-packaging/
//
// Hard
//
// You have n packages that you are trying to place in boxes, one package in each box. There are m suppliers that each produce boxes of different sizes (with infinite supply). A package can be placed in a box if the size of the package is less than or equal to the size of the box.

The package sizes are given as an integer array packages, where packages[i] is the size of the ith package. The suppliers are given as a 2D integer array boxes, where boxes[j] is an array of box sizes that the jth supplier produces.

You want to choose a single supplier and use boxes from them such that the total wasted space is minimized. For each package in a box, we define the space wasted to be size of the box - size of the package. The total wasted space is the sum of the space wasted in all the boxes.

    For example, if you have to fit packages with sizes [2,3,5] and the supplier offers boxes of sizes [4,8], you can fit the packages of size-2 and size-3 into two boxes of size-4 and the package with size-5 into a box of size-8. This would result in a waste of (4-2) + (4-3) + (8-5) = 6.

Return the minimum total wasted space by choosing the box supplier optimally, or -1 if it is impossible to fit all the packages inside boxes. Since the answer may be large, return it modulo 109 + 7.

Example 1:

Input: packages = [2,3,5], boxes = [[4,8],[2,8]]
Output: 6
Explanation: It is optimal to choose the first supplier, using two size-4 boxes and one size-8 box.
The total waste is (4-2) + (4-3) + (8-5) = 6.

Example 2:

Input: packages = [2,3,5], boxes = [[1,4],[2,3],[3,4]]
Output: -1
Explanation: There is no box that the package of size 5 can fit in.

Example 3:

Input: packages = [3,5,8,10,11,12], boxes = [[12],[11,9],[10,5,14]]
Output: 9
Explanation: It is optimal to choose the third supplier, using two size-5 boxes, two size-10 boxes, and two size-14 boxes.
The total waste is (5-3) + (5-5) + (10-8) + (10-10) + (14-11) + (14-12) = 9.

Constraints:

    n == packages.length
    m == boxes.length
    1 <= n <= 10^5
    1 <= m <= 10^5
    1 <= packages[i] <= 10^5
    1 <= boxes[j].length <= 10^5
    1 <= boxes[j][k] <= 10^5
    sum(boxes[j].length) <= 10^5
    The elements in boxes[j] are distinct.
*/

struct Solution;

impl Solution {
    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        let mut packages = packages.iter().map(|&x| x as i64).collect::<Vec<_>>();
        packages.sort();
        let f = |x: &Vec<i32>| x.iter().map(|&y| y as i64).collect::<Vec<_>>();
        let mut boxes = boxes.iter().map(f).collect::<Vec<_>>();
        let mut res = i64::MAX;
        let sum_a = packages.iter().sum::<i64>();
        for box_ in boxes.iter_mut() {
            box_.sort();
            if box_[box_.len() - 1] < packages[packages.len() - 1] {
                continue;
            }
            let mut cur = 0;
            let mut i = 0;
            for &b in box_.iter() {
                let j = upper_bound(&packages[i..], &b).unwrap_or_else(|x| x) + i;
                cur += b * (j - i) as i64;
                i = j;
            }
            res = res.min(cur);
        }
        if res < i64::MAX { ((res - sum_a) % 1_000_000_007) as _ } else { -1 }
    }
}

pub fn upper_bound<T: Ord>(coll: &[T], x: &T) -> Result<usize, usize> {
    let len = coll.len();
    let position = coll.iter().position(|e| e > x);
    let idx = coll.binary_search_by(|e| e.cmp(x));
    match position {
        Some(index) => Ok(index),
        None => match idx {
            Ok(_) => Err(len),
            Err(idx) => {
                if idx == len {
                    Err(len)
                } else {
                    Ok(idx)
                }
            }
        },
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 3, 5], vec![vec![4, 8], vec![2, 8]], 6),
        (vec![2, 3, 5], vec![vec![1, 4], vec![2, 3], vec![3, 4]], -1),
        (vec![3, 5, 8, 10, 11, 12], vec![vec![12], vec![11, 9], vec![10, 5, 14]], 9),
    ];
    for (packages, boxes, expected) in cases {
        assert_eq!(Solution::min_wasted_space(packages, boxes), expected);
    }
}
