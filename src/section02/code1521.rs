#![allow(dead_code)]

/*

// 1521. Find a Value of a Mysterious Function Closest to Target
// https://leetcode.com/problems/find-a-value-of-a-mysterious-function-closest-to-target/
// https://leetcode.cn/problems/find-a-value-of-a-mysterious-function-closest-to-target/
//
// Hard
//
// Winston was given the above mysterious function func. He has an integer array arr and an integer target and he wants to find the values l and r that make the value |func(arr, l, r) - target| minimum possible.

Return the minimum possible value of |func(arr, l, r) - target|.

Notice that func should be called with the values l and r where 0 <= l, r < arr.length.

Example 1:

Input: arr = [9,12,3,7,15], target = 5
Output: 2
Explanation: Calling func with all the pairs of [l,r] = [[0,0],[1,1],[2,2],[3,3],[4,4],[0,1],[1,2],[2,3],[3,4],[0,2],[1,3],[2,4],[0,3],[1,4],[0,4]], Winston got the following results [9,12,3,7,15,8,0,3,7,0,0,3,0,0,0]. The value closest to 5 is 7 and 3, thus the minimum difference is 2.

Example 2:

Input: arr = [1000000,1000000,1000000], target = 1
Output: 999999
Explanation: Winston called the func with all possible values of [l,r] and he always got 1000000, thus the min difference is 999999.

Example 3:

Input: arr = [1,2,4,8,16], target = 0
Output: 0

Constraints:

    1 <= arr.length <= 10^5
    1 <= arr[i] <= 10^6
    0 <= target <= 10^7
*/

struct Solution;

impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let len = arr.len();
        let mut tab = vec![vec![]; 30];
        for (i, &item0) in arr.iter().enumerate() {
            for (j, item) in tab.iter_mut().enumerate() {
                if (item0 >> j) & 1 == 1 {
                    item.push(i);
                }
            }
        }

        let mut dp = vec![0; len];
        let mut ans = (arr[0] - target).abs();
        for i in (0..len).rev() {
            for j in 0..30 {
                if (arr[i] >> j) & 1 == 0 {
                    while !tab[j].is_empty() && tab[j].last().unwrap() > &i {
                        dp[*tab[j].last().unwrap()] -= 1 << j;
                        tab[j].pop();
                    }
                }
            }
            dp[i] = arr[i];
            let mut l = i;
            let mut r = len;
            while l < r {
                let m = l + (r - l) / 2;
                if dp[m] > target {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            if l == len {
                ans = ans.min((target - dp[l - 1]).abs());
            } else if l == i {
                ans = ans.min((target - dp[l]).abs());
            } else {
                ans = ans.min((target - dp[l]).abs().min((target - dp[l - 1]).abs()));
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![9, 12, 3, 7, 15], 5, 2),
        (vec![1000000, 1000000, 1000000], 1, 999999),
        (vec![1, 2, 4, 8, 16], 0, 0),
    ];
    for (arr, target, expected) in cases {
        assert_eq!(Solution::closest_to_target(arr, target), expected);
    }
}
