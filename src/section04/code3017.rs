#![allow(dead_code)]

// 3017. Count the Number of Houses at a Certain Distance II
// https://leetcode.com/problems/count-the-number-of-houses-at-a-certain-distance-ii/
// https://leetcode.cn/problems/count-the-number-of-houses-at-a-certain-distance-ii/
//
// Hard
//
// You are given three positive integers n, x, and y.
//
// In a city, there exist houses numbered 1 to n connected by n streets. There is a street connecting the house
// numbered i with the house numbered i + 1 for all 1 <= i <= n - 1 .
// An additional street connects the house numbered x with the house numbered y.
//
// For each k, such that 1 <= k <= n, you need to find the number of pairs of houses (house1, house2) such
// that the minimum number of streets that need to be traveled to reach house2 from house1 is k.
//
// Return a 1-indexed array result of length n where result[k] represents the total number of pairs of houses
// such that the minimum streets required to reach one house from the other is k.
//
// Note that x and y can be equal.
//
// Example 1:
//
// Input: n = 3, x = 1, y = 3
// Output: [6,0,0]
// Explanation: Let's look at each pair of houses:
// - For the pair (1, 2), we can go from house 1 to house 2 directly.
// - For the pair (2, 1), we can go from house 2 to house 1 directly.
// - For the pair (1, 3), we can go from house 1 to house 3 directly.
// - For the pair (3, 1), we can go from house 3 to house 1 directly.
// - For the pair (2, 3), we can go from house 2 to house 3 directly.
// - For the pair (3, 2), we can go from house 3 to house 2 directly.
//
// Example 2:
//
// Input: n = 5, x = 2, y = 4
// Output: [10,8,2,0,0]
// Explanation: For each distance k the pairs are:
// - For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (2, 4), (4, 2), (3, 4), (4, 3), (4, 5), and (5, 4).
// - For k == 2, the pairs are (1, 3), (3, 1), (1, 4), (4, 1), (2, 5), (5, 2), (3, 5), and (5, 3).
// - For k == 3, the pairs are (1, 5), and (5, 1).
// - For k == 4 and k == 5, there are no pairs.
//
// Example 3:
//
// Input: n = 4, x = 1, y = 1
// Output: [6,4,2,0]
// Explanation: For each distance k the pairs are:
// - For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (3, 4), and (4, 3).
// - For k == 2, the pairs are (1, 3), (3, 1), (2, 4), and (4, 2).
// - For k == 3, the pairs are (1, 4), and (4, 1).
// - For k == 4, there are no pairs.
//
// Constraints:
//
// 2 <= n <= 10^5
// 1 <= x, y <= n
//

struct Solution;

impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let (mut x, mut y) = (x, y);
        if y < x {
            std::mem::swap(&mut x, &mut y);
        }
        let mut ans = vec![0_i64; n as usize];
        let cycle = y - x + 1;
        let half_cycle = (cycle - 1) >> 1;
        let tail1 = x - 1;
        let tail2 = n - y;
        let tail_to_tail = tail1 + tail2 + 1 + (x != y) as i32;
        for i in 0..half_cycle {
            ans[i as usize] = cycle as i64 * 2;
        }
        if cycle % 2 == 0 {
            ans[half_cycle as usize] = cycle as i64;
        }
        for i in 1..tail_to_tail {
            ans[(i - 1) as usize] += (tail_to_tail - i) as i64 * 2;
        }
        if x != y {
            ans[0] -= 2;
            for &tail in &[tail1, tail2] {
                for i in 1..=tail {
                    ans[i as usize] -= 2;
                }
            }
        }
        for &tail in &[tail1, tail2] {
            for i in 1..tail + half_cycle {
                ans[i as usize] += 4 * tail.min(half_cycle).min(i).min(tail + half_cycle - i) as i64;
            }
            if cycle % 2 == 0 {
                for i in 1..=tail {
                    ans[(i + half_cycle) as usize] += 2;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let n = 3;
    let x = 1;
    let y = 3;
    let output = vec![6, 0, 0];
    assert_eq!(Solution::count_of_pairs(n, x, y), output);

    let n = 5;
    let x = 2;
    let y = 4;
    let output = vec![10, 8, 2, 0, 0];
    assert_eq!(Solution::count_of_pairs(n, x, y), output);

    let n = 4;
    let x = 1;
    let y = 1;
    let output = vec![6, 4, 2, 0];
    assert_eq!(Solution::count_of_pairs(n, x, y), output);
}
