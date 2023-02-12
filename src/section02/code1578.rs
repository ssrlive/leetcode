#![allow(dead_code)]

/*

// 1578. Minimum Time to Make Rope Colorful
Medium
2.7K
79
Companies

Alice has n balloons arranged on a rope. You are given a 0-indexed string colors where colors[i] is the color of the ith balloon.

Alice wants the rope to be colorful. She does not want two consecutive balloons to be of the same color, so she asks Bob for help. Bob can remove some balloons from the rope to make it colorful. You are given a 0-indexed integer array neededTime where neededTime[i] is the time (in seconds) that Bob needs to remove the ith balloon from the rope.

Return the minimum time Bob needs to make the rope colorful.

Example 1:

Input: colors = "abaac", neededTime = [1,2,3,4,5]
Output: 3
Explanation: In the above image, 'a' is blue, 'b' is red, and 'c' is green.
Bob can remove the blue balloon at index 2. This takes 3 seconds.
There are no longer two consecutive balloons of the same color. Total time = 3.

Example 2:

Input: colors = "abc", neededTime = [1,2,3]
Output: 0
Explanation: The rope is already colorful. Bob does not need to remove any balloons from the rope.

Example 3:

Input: colors = "aabaa", neededTime = [1,2,3,4,1]
Output: 2
Explanation: Bob will remove the ballons at indices 0 and 4. Each ballon takes 1 second to remove.
There are no longer two consecutive balloons of the same color. Total time = 1 + 1 = 2.

Constraints:

    n == colors.length == neededTime.length
    1 <= n <= 105
    1 <= neededTime[i] <= 104
    colors contains only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut rez = 0;

        let mut start = 0;
        let colors = colors.as_bytes();
        let mut change_rez = |v: &[i32]| {
            if v.len() > 1 {
                rez += v.iter().sum::<i32>() - v.iter().max().unwrap();
            }
        };
        for (i, _) in colors.windows(2).enumerate().filter(|(_, w)| w[0] != w[1]) {
            change_rez(&needed_time[start..=i]);
            start = i + 1;
        }
        change_rez(&needed_time[start..]);
        rez
    }
}

#[test]
fn test() {
    let colors = "abaac".to_string();
    let needed_time = vec![1, 2, 3, 4, 5];
    let res = 3;
    assert_eq!(Solution::min_cost(colors, needed_time), res);
    let colors = "abc".to_string();
    let needed_time = vec![1, 2, 3];
    let res = 0;
    assert_eq!(Solution::min_cost(colors, needed_time), res);
    let colors = "aabaa".to_string();
    let needed_time = vec![1, 2, 3, 4, 1];
    let res = 2;
    assert_eq!(Solution::min_cost(colors, needed_time), res);
}
