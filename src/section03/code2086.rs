#![allow(dead_code)]

/*

// 2086. Minimum Number of Food Buckets to Feed the Hamsters
// https://leetcode.com/problems/minimum-number-of-food-buckets-to-feed-the-hamsters/
// https://leetcode.cn/problems/minimum-number-of-food-buckets-to-feed-the-hamsters/
//
// Medium
//
// You are given a 0-indexed string hamsters where hamsters[i] is either:

    'H' indicating that there is a hamster at index i, or
    '.' indicating that index i is empty.

You will add some number of food buckets at the empty indices in order to feed the hamsters. A hamster can be fed if there is at least one food bucket to its left or to its right. More formally, a hamster at index i can be fed if you place a food bucket at index i - 1 and/or at index i + 1.

Return the minimum number of food buckets you should place at empty indices to feed all the hamsters or -1 if it is impossible to feed all of them.

Example 1:

Input: hamsters = "H..H"
Output: 2
Explanation: We place two food buckets at indices 1 and 2.
It can be shown that if we place only one food bucket, one of the hamsters will not be fed.

Example 2:

Input: hamsters = ".H.H."
Output: 1
Explanation: We place one food bucket at index 2.

Example 3:

Input: hamsters = ".HHH."
Output: -1
Explanation: If we place a food bucket at every empty index as shown, the hamster at index 2 will not be able to eat.

Constraints:

    1 <= hamsters.length <= 10^5
    hamsters[i] is either'H' or '.'.
*/

struct Solution;

impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut s = hamsters.chars().collect::<Vec<char>>();
        s.insert(0, 'H');
        s.push('H');
        let n = s.len();
        for i in 1..n - 1 {
            if s[i - 1] == 'H' && s[i] == 'H' && s[i + 1] == 'H' {
                return -1;
            }
        }
        s[0] = '.';
        s[n - 1] = '.';
        let mut count = 0;
        for i in 1..n - 1 {
            if s[i - 1] == 'H' && s[i] == '.' && s[i + 1] == 'H' {
                count += 1;
                s[i - 1] = '.';
                s[i + 1] = '.';
            }
        }
        for &s_i in s.iter().take(n - 1).skip(1) {
            if s_i == 'H' {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("H..H", 2),
        (".H.H.", 1),
        (".HHH.", -1),
    ];
    for (hamsters, expected) in cases {
        assert_eq!(Solution::minimum_buckets(hamsters.to_string()), expected);
    }
}
