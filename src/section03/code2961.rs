#![allow(dead_code)]

// 2961. Double Modular Exponentiation
// https://leetcode.com/problems/double-modular-exponentiation/
// https://leetcode.cn/problems/double-modular-exponentiation/
//
// Medium
//
// You are given a 0-indexed 2D array variables where variables[i] = [ai, bi, ci, mi], and an integer target.
//
// An index i is good if the following formula holds:
//
//     0 <= i < variables.length
//     ((aibi % 10)ci) % mi == target
//
// Return an array consisting of good indices in any order.
//
// Example 1:
//
// Input: variables = [[2,3,3,10],[3,3,3,1],[6,1,1,4]], target = 2
// Output: [0,2]
// Explanation: For each index i in the variables array:
// 1) For the index 0, variables[0] = [2,3,3,10], (23 % 10)3 % 10 = 2.
// 2) For the index 1, variables[1] = [3,3,3,1], (33 % 10)3 % 1 = 0.
// 3) For the index 2, variables[2] = [6,1,1,4], (61 % 10)1 % 4 = 2.
// Therefore we return [0,2] as the answer.
//
// Example 2:
//
// Input: variables = [[39,3,1000,1000]], target = 17
// Output: []
// Explanation: For each index i in the variables array:
// 1) For the index 0, variables[0] = [39,3,1000,1000], (393 % 10)1000 % 1000 = 1.
// Therefore we return [] as the answer.
//
// Constraints:
//
//     1 <= variables.length <= 100
//     variables[i] == [ai, bi, ci, mi]
//     1 <= ai, bi, ci, mi <= 10^3
//     0 <= target <= 10^3
//

struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        (0..variables.len())
            .filter(|&i| {
                let (a, b, c, m) = (variables[i][0], variables[i][1], variables[i][2], variables[i][3]);
                let x = Self::pow(a, b as usize, 10);
                let y = Self::pow(x, c as usize, m);
                y == target
            })
            .map(|i| i as i32)
            .collect()
    }

    fn pow(a: i32, b: usize, m: i32) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut stack = Vec::new();
        let mut last = 1;

        while set.insert(last) {
            stack.push(last);
            last = (last * a) % m;
        }

        let n = stack.len();
        let cycle = stack.iter().position(|&x| x == last).unwrap();

        if b < n {
            stack[b]
        } else {
            stack[cycle + (b - cycle) % (n - cycle)]
        }
    }
}

#[test]
fn test() {
    let variables = vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]];
    let target = 2;
    let res = vec![0, 2];
    assert_eq!(Solution::get_good_indices(variables, target), res);

    let variables = vec![vec![39, 3, 1000, 1000]];
    let target = 17;
    let res = vec![];
    assert_eq!(Solution::get_good_indices(variables, target), res);
}
