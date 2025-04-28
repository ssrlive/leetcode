#![allow(dead_code)]

// 28. Unit Conversion I
// https://leetcode.com/problems/unit-conversion-i/
// https://leetcode.cn/problems/unit-conversion-i/
//
// Medium
//
// There are n types of units indexed from 0 to n - 1. You are given a 2D integer array conversions of length n - 1, where conversions[i] = [sourceUniti, targetUniti, conversionFactori]. This indicates that a single unit of type sourceUniti is equivalent to conversionFactori units of type targetUniti.
//
// Return an array baseUnitConversion of length n, where baseUnitConversion[i] is the number of units of type i equivalent to a single unit of type 0. Since the answer may be large, return each baseUnitConversion[i] modulo 109 + 7.
//
// Example 1:
//
// Input: conversions = [[0,1,2],[1,2,3]]
//
// Output: [1,2,6]
//
// Explanation:
//
//     Convert a single unit of type 0 into 2 units of type 1 using conversions[0].
//     Convert a single unit of type 0 into 6 units of type 2 using conversions[0], then conversions[1].
//
// Example 2:
//
// Input: conversions = [[0,1,2],[0,2,3],[1,3,4],[1,4,5],[2,5,2],[4,6,3],[5,7,4]]
//
// Output: [1,2,3,8,10,6,30,24]
//
// Explanation:
//
//     Convert a single unit of type 0 into 2 units of type 1 using conversions[0].
//     Convert a single unit of type 0 into 3 units of type 2 using conversions[1].
//     Convert a single unit of type 0 into 8 units of type 3 using conversions[0], then conversions[2].
//     Convert a single unit of type 0 into 10 units of type 4 using conversions[0], then conversions[3].
//     Convert a single unit of type 0 into 6 units of type 5 using conversions[1], then conversions[4].
//     Convert a single unit of type 0 into 30 units of type 6 using conversions[0], conversions[3], then conversions[5].
//     Convert a single unit of type 0 into 24 units of type 7 using conversions[1], conversions[4], then conversions[6].
//
// Constraints:
//
//     2 <= n <= 105
//     conversions.length == n - 1
//     0 <= sourceUniti, targetUniti < n
//     1 <= conversionFactori <= 109
//     It is guaranteed that unit 0 can be converted into any other unit through a unique combination of conversions without using any conversions in the opposite direction.
//

struct Solution;

impl Solution {
    pub fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = conversions.len() + 1;
        let mut g = vec![vec![]; n];
        for v in conversions {
            g[v[0] as usize].push((v[1], v[2]));
        }

        let mut ans = vec![0; n];
        const MOD: i64 = 1_000_000_007;
        let mut q = std::collections::VecDeque::new();
        ans[0] = 1;
        q.push_back(0);

        while let Some(x) = q.pop_front() {
            for &(y, z) in &g[x as usize] {
                ans[y as usize] = (ans[x as usize] * z as i64) % MOD;
                q.push_back(y);
            }
        }

        ans.iter().map(|&x| x as i32).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::base_unit_conversions(vec![vec![0, 1, 2], vec![1, 2, 3]]), vec![1, 2, 6]);
    assert_eq!(
        Solution::base_unit_conversions(vec![
            vec![0, 1, 2],
            vec![0, 2, 3],
            vec![1, 3, 4],
            vec![1, 4, 5],
            vec![2, 5, 2],
            vec![4, 6, 3],
            vec![5, 7, 4]
        ]),
        vec![1, 2, 3, 8, 10, 6, 30, 24]
    );

    assert_eq!(
        Solution::base_unit_conversions(vec![vec![0, 1, 1_000_000_000], vec![1, 2, 1_000_000_000]]),
        vec![1, 1_000_000_000, 49]
    );
}
