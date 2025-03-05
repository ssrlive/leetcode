#![allow(dead_code)]

// 3145. Find Products of Elements of Big Array
// https://leetcode.com/problems/find-products-of-elements-of-big-array/
// https://leetcode.cn/problems/find-products-of-elements-of-big-array/
//
// Hard
//
// A powerful array for an integer x is the shortest sorted array of powers of two that sum up to x.
// For example, the powerful array for 11 is [1, 2, 8].
//
// The array big_nums is created by concatenating the powerful arrays for every positive integer i in ascending order: 1, 2, 3, and so forth.
// Thus, big_nums starts as [1, 2, 1, 2, 4, 1, 4, 2, 4, 1, 2, 4, 8, ...].
//
// You are given a 2D integer matrix queries, where for queries[i] = [fromi, toi, modi] you should
// calculate (big_nums[fromi] * big_nums[fromi + 1] * ... * big_nums[toi]) % modi.
//
// Return an integer array answer such that answer[i] is the answer to the ith query.
//
// Example 1:
//
// Input: queries = [[1,3,7]]
//
// Output: [4]
//
// Explanation:
//
// There is one query.
//
// big_nums[1..3] = [2,1,2]. The product of them is 4. The remainder of 4 under 7 is 4.
//
// Example 2:
//
// Input: queries = [[2,5,3],[7,7,4]]
//
// Output: [2,2]
//
// Explanation:
//
// There are two queries.
//
// First query: big_nums[2..5] = [1,2,4,1]. The product of them is 8. The remainder of 8 under 3 is 2.
//
// Second query: big_nums[7] = 2. The remainder of 2 under 4 is 2.
//
// Constraints:
//
// 1 <= queries.length <= 500
// queries[i].length == 3
// 0 <= queries[i][0] <= queries[i][1] <= 10^15
// 1 <= queries[i][2] <= 10^5
//

struct Solution;

impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        let mx = 1_000_000_000_000_006;
        let lim = 60;
        let mut ret = vec![];

        let mul = |x: i64, y: i64, m: i64| -> i64 {
            let x = x % m;
            let y = y % m;
            let res = x * y;
            if res >= m { res % m } else { res }
        };

        let mod_pow = |x: i64, y: i64, m: i64| -> i64 {
            if y <= 0 {
                return 1;
            }
            let mut ans = 1;
            let mut x = x % m;
            let mut y = y;
            while y != 0 {
                if y & 1 == 1 {
                    ans = mul(ans, x, m);
                }
                x = mul(x, x, m);
                y >>= 1;
            }
            ans
        };

        let calc = |n: i64| -> i64 {
            if n <= 0 {
                return n;
            }
            let mut lo = 1;
            let mut hi = mx;
            while lo <= hi {
                let mid = (lo + hi) / 2;
                let mut sum = 0;
                for i in 0..lim {
                    let p = 1_i64 << (i + 1);
                    let mut cur = ((mid + 1) / p) * (1_i64 << i);
                    cur += 0_i64.max((mid + 1) % p - (1_i64 << i));
                    sum += cur;
                }
                if sum >= n {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }
            hi + 1
        };

        let calc2 = |n: i64, tot: i64| -> Vec<i64> {
            let mut cnt = vec![0; lim];
            if n <= 0 {
                return cnt;
            }
            let mut sum = 0;
            for (i, cnt_i) in cnt.iter_mut().enumerate().take(lim) {
                let p = 1_i64 << (i + 1);
                let mut cur = (n / p) * (1_i64 << i);
                cur += 0_i64.max(n % p - (1_i64 << i));
                sum += cur;
                *cnt_i = cur;
            }
            if sum < tot {
                for (i, cnt_i) in cnt.iter_mut().enumerate().take(lim) {
                    if sum >= tot {
                        break;
                    }
                    if (n >> i) & 1 == 1 {
                        *cnt_i += 1;
                        sum += 1;
                    }
                }
            }
            cnt
        };

        for q in queries {
            let v1 = calc2(calc(q[1] + 1), q[1] + 1);
            let v2 = calc2(calc(q[0]), q[0]);

            let mut val = 1;
            for i in 0..lim {
                let p = 1_i64 << i;
                let temp = mod_pow(p, v1[i] - v2[i], q[2]);
                val = (val * temp) % q[2];
            }
            ret.push(val as i32);
        }
        ret
    }
}

#[test]
fn test() {
    let queries = vec![vec![1, 3, 7]];
    let output = vec![4];
    assert_eq!(Solution::find_products_of_elements(queries), output);

    let queries = vec![vec![2, 5, 3], vec![7, 7, 4]];
    let output = vec![2, 2];
    assert_eq!(Solution::find_products_of_elements(queries), output);
}
