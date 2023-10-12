#![allow(dead_code)]

// 2672. Number of Adjacent Elements With the Same Color
// https://leetcode.com/problems/number-of-adjacent-elements-with-the-same-color/
// https://leetcode.cn/problems/number-of-adjacent-elements-with-the-same-color/
//
// Medium
//
// There is a 0-indexed array nums of length n. Initially, all elements are uncolored (has a value of 0).
//
// You are given a 2D integer array queries where queries[i] = [indexi, colori].
//
// For each query, you color the index indexi with the color colori in the array nums.
//
// Return an array answer of the same length as queries where answer[i] is the number of adjacent elements
// with the same color after the ith query.
//
// More formally, answer[i] is the number of indices j, such that 0 <= j < n - 1 and nums[j] == nums[j + 1]
// and nums[j] != 0 after the i^th query.
//
// Example 1:
//
// Input: n = 4, queries = [[0,2],[1,2],[3,1],[1,1],[2,1]]
// Output: [0,1,1,0,2]
// Explanation: Initially array nums = [0,0,0,0], where 0 denotes uncolored elements of the array.
// - After the 1st query nums = [2,0,0,0]. The count of adjacent elements with the same color is 0.
// - After the 2nd query nums = [2,2,0,0]. The count of adjacent elements with the same color is 1.
// - After the 3rd query nums = [2,2,0,1]. The count of adjacent elements with the same color is 1.
// - After the 4th query nums = [2,1,0,1]. The count of adjacent elements with the same color is 0.
// - After the 5th query nums = [2,1,1,1]. The count of adjacent elements with the same color is 2.
//
// Example 2:
//
// Input: n = 1, queries = [[0,100000]]
// Output: [0]
// Explanation: Initially array nums = [0], where 0 denotes uncolored elements of the array.
// - After the 1st query nums = [100000]. The count of adjacent elements with the same color is 0.
//
// Constraints:
//
//     1 <= n <= 10^5
//     1 <= queries.length <= 10^5
//     queries[i].length == 2
//     0 <= indexi <= n - 1
//     1 <=  colori <= 10^5
//

struct Solution;

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![0; n as usize];
        let mut result = vec![];
        let mut temp = 0;

        for query in queries.iter() {
            let index = query[0];
            let color = query[1];

            for index_graph in [index - 1, index + 1] {
                // valid index between 0 ~ n-1
                if index_graph < 0 || index_graph >= graph.len() as _ {
                    continue;
                }
                let index_graph = index_graph as usize;
                let index = index as usize;

                // original color isn't 0 && new color != old color
                if graph[index] != 0 && color != graph[index] {
                    // before colored: 2 neighbor colors both equal
                    if graph[index_graph] == graph[index] {
                        temp -= 1;
                    }
                    // after colored: 2 neighbor colors both equal
                    else if graph[index_graph] == color {
                        temp += 1;
                    }
                }
                // original color is 0 && color in both positions are equal after colored
                else if graph[index] == 0 && graph[index_graph] == color {
                    temp += 1;
                }
            }
            result.push(temp);

            graph[index as usize] = color;
        }

        result
    }
}

#[test]
fn test() {
    let v = vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]];
    assert_eq!(Solution::color_the_array(4, v), vec![0, 1, 1, 0, 2]);

    assert_eq!(Solution::color_the_array(1, vec![vec![0, 100000]]), vec![0]);
}
