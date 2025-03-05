#![allow(dead_code)]

// 3470. Permutations IV
// https://leetcode.com/problems/permutations-iv/
// https://leetcode.cn/problems/permutations-iv/
//
// Hard
//
// Given two integers, n and k, an alternating permutation is a permutation of the first n positive integers such that no two adjacent elements are both odd or both even.
//
// Return the k-th alternating permutation sorted in lexicographical order. If there are fewer than k valid alternating permutations, return an empty list.
//
// Example 1:
//
// Input: n = 4, k = 6
//
// Output: [3,4,1,2]
//
// Explanation:
//
// The lexicographically-sorted alternating permutations of [1, 2, 3, 4] are:
//
// [1, 2, 3, 4]
// [1, 4, 3, 2]
// [2, 1, 4, 3]
// [2, 3, 4, 1]
// [3, 2, 1, 4]
// [3, 4, 1, 2] ← 6th permutation
// [4, 1, 2, 3]
// [4, 3, 2, 1]
// Since k = 6, we return [3, 4, 1, 2].
//
// Example 2:
//
// Input: n = 3, k = 2
//
// Output: [3,2,1]
//
// Explanation:
//
// The lexicographically-sorted alternating permutations of [1, 2, 3] are:
//
// [1, 2, 3]
// [3, 2, 1] ← 2nd permutation
// Since k = 2, we return [3, 2, 1].
//
// Example 3:
//
// Input: n = 2, k = 3
//
// Output: []
//
// Explanation:
//
// The lexicographically-sorted alternating permutations of [1, 2] are:
//
// [1, 2]
// [2, 1]
// There are only 2 alternating permutations, but k = 3, which is out of range. Thus, we return an empty list [].
//
// Constraints:
//
// 1 <= n <= 100
// 1 <= k <= 10^15
//

struct Solution;

impl Solution {
    pub fn permute(n: i32, k: i64) -> Vec<i32> {
        const INF: i64 = 1_000_000_000_000_000_000;
        fn helper(a: i64, b: i64) -> i64 {
            let mut res = 1;
            for i in 0..b {
                res *= a - i;
                if res > INF {
                    return INF;
                }
            }
            res
        }
        fn solve(odd: i64, even: i64, r: i64, req: i64) -> i64 {
            if r == 0 {
                return 1;
            }
            let nodd;
            let neven;
            if req == 1 {
                nodd = (r + 1) / 2;
                neven = r / 2;
            } else {
                neven = (r + 1) / 2;
                nodd = r / 2;
            }
            if odd < nodd || even < neven {
                return 0;
            }
            let oddways = helper(odd, nodd);
            let evenways = helper(even, neven);
            let mut total = oddways;
            if evenways == 0 || total > INF / evenways {
                total = INF;
            } else {
                total *= evenways;
            }
            total
        }

        let mut ans = Vec::new();
        let mut first = false;
        let mut used = vec![false; n as usize + 1];
        let mut odd = (n + 1) / 2;
        let mut even = n / 2;
        let mut last = 0;
        let mut k = k;
        for i in 1..=n {
            if used[i as usize] {
                continue;
            }
            let mut odd2 = odd;
            let mut even2 = even;
            let cp = i & 1;
            let next = if cp == 1 { 0 } else { 1 };
            if cp == 1 {
                odd2 -= 1;
            } else {
                even2 -= 1;
            }
            let r = n - 1;
            let cnt = solve(odd2 as _, even2 as _, r as _, next);
            if k > cnt {
                k -= cnt;
            } else {
                ans.push(i);
                used[i as usize] = true;
                odd = odd2;
                even = even2;
                last = cp;
                first = true;
                break;
            }
        }
        if !first {
            return Vec::new();
        }
        for z in 1..n {
            let mut taken = false;
            for j in 1..=n {
                if !used[j as usize] && (j & 1) != last {
                    let mut odd2 = odd;
                    let mut even2 = even;
                    let cp = j & 1;
                    if cp == 1 {
                        odd2 -= 1;
                    } else {
                        even2 -= 1;
                    }
                    let r = n - (z + 1);
                    let next = if cp == 1 { 0 } else { 1 };
                    let cnt2 = solve(odd2 as i64, even2 as i64, r as i64, next);
                    if k > cnt2 {
                        k -= cnt2;
                    } else {
                        ans.push(j);
                        used[j as usize] = true;
                        odd = odd2;
                        even = even2;
                        last = cp;
                        taken = true;
                        break;
                    }
                }
            }
            if !taken {
                return Vec::new();
            }
        }
        ans
    }
}

#[test]
fn test() {
    let n = 4;
    let k = 6;
    let output = vec![3, 4, 1, 2];
    assert_eq!(Solution::permute(n, k), output);

    let n = 3;
    let k = 2;
    let output = vec![3, 2, 1];
    assert_eq!(Solution::permute(n, k), output);

    let n = 2;
    let k = 3;
    let output = Vec::<i32>::new();
    assert_eq!(Solution::permute(n, k), output);
}
