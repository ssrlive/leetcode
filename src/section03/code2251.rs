#![allow(dead_code)]

/*

// 2251. Number of Flowers in Full Bloom
// https://leetcode.com/problems/number-of-flowers-in-full-bloom/
// https://leetcode.cn/problems/number-of-flowers-in-full-bloom/
//
// Hard
//
// You are given a 0-indexed 2D integer array flowers, where flowers[i] = [starti, endi] means the ith flower will be in full bloom from starti to endi (inclusive). You are also given a 0-indexed integer array persons of size n, where persons[i] is the time that the ith person will arrive to see the flowers.

Return an integer array answer of size n, where answer[i] is the number of flowers that are in full bloom when the ith person arrives.

Example 1:

Input: flowers = [[1,6],[3,7],[9,12],[4,13]], persons = [2,3,7,11]
Output: [1,2,2,2]
Explanation: The figure above shows the times when the flowers are in full bloom and when the people arrive.
For each person, we return the number of flowers in full bloom during their arrival.

Example 2:

Input: flowers = [[1,10],[3,3]], persons = [3,3,2]
Output: [2,2,1]
Explanation: The figure above shows the times when the flowers are in full bloom and when the people arrive.
For each person, we return the number of flowers in full bloom during their arrival.

Constraints:

    1 <= flowers.length <= 5 * 10^4
    flowers[i].length == 2
    1 <= starti <= endi <= 10^9
    1 <= persons.length <= 5 * 10^4
    1 <= persons[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<(i32, usize)> = vec![];
        for f in flowers {
            let start = f[0];
            let end = f[1];
            arr.push((start, 0_usize));
            arr.push((end, usize::MAX));
        }
        for (i, p) in persons.iter().enumerate() {
            arr.push((*p, i + 1));
        }
        arr.sort();
        let n = persons.len();
        let mut res = vec![0; n];
        let mut cnt = 0;
        for (_, i) in arr {
            if i == 0 {
                cnt += 1;
            } else if i == usize::MAX {
                cnt -= 1;
            } else {
                res[i - 1] = cnt
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
            vec![2, 3, 7, 11],
            vec![1, 2, 2, 2],
        ),
        (vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2], vec![2, 2, 1]),
    ];
    for (flowers, persons, expect) in cases {
        assert_eq!(Solution::full_bloom_flowers(flowers, persons), expect);
    }
}
