#![allow(dead_code)]

/*

// 1953. Maximum Number of Weeks for Which You Can Work
// https://leetcode.com/problems/maximum-number-of-weeks-for-which-you-can-work/
// https://leetcode.cn/problems/maximum-number-of-weeks-for-which-you-can-work/
//
// Medium
//
// There are n projects numbered from 0 to n - 1. You are given an integer array milestones where each milestones[i] denotes the number of milestones the ith project has.

You can work on the projects following these two rules:

    Every week, you will finish exactly one milestone of one project. You must work every week.
    You cannot work on two milestones from the same project for two consecutive weeks.

Once all the milestones of all the projects are finished, or if the only milestones that you can work on will cause you to violate the above rules, you will stop working. Note that you may not be able to finish every project's milestones due to these constraints.

Return the maximum number of weeks you would be able to work on the projects without violating the rules mentioned above.

Example 1:

Input: milestones = [1,2,3]
Output: 6
Explanation: One possible scenario is:
​​​​- During the 1st week, you will work on a milestone of project 0.
- During the 2nd week, you will work on a milestone of project 2.
- During the 3rd week, you will work on a milestone of project 1.
- During the 4th week, you will work on a milestone of project 2.
- During the 5th week, you will work on a milestone of project 1.
- During the 6th week, you will work on a milestone of project 2.
The total number of weeks is 6.

Example 2:

Input: milestones = [5,2,1]
Output: 7
Explanation: One possible scenario is:
- During the 1st week, you will work on a milestone of project 0.
- During the 2nd week, you will work on a milestone of project 1.
- During the 3rd week, you will work on a milestone of project 0.
- During the 4th week, you will work on a milestone of project 1.
- During the 5th week, you will work on a milestone of project 0.
- During the 6th week, you will work on a milestone of project 2.
- During the 7th week, you will work on a milestone of project 0.
The total number of weeks is 7.
Note that you cannot work on the last milestone of project 0 on 8th week because it would violate the rules.
Thus, one milestone in project 0 will remain unfinished.

Constraints:

    n == milestones.length
    1 <= n <= 10^5
    1 <= milestones[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn is_three(n: i32) -> bool {
        match n {
            4 => return true,
            v if v < 4 || v % 2 == 0 => return false,
            _ => (),
        }
        let i32_sqrt = |x| -> i32 { (x as f32).sqrt() as i32 };
        let sq = i32_sqrt(n);
        if sq * sq != n {
            return false;
        }
        for i in (3..i32_sqrt(sq) + 1).step_by(2) {
            if sq % i == 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_three(2), false);
    assert_eq!(Solution::is_three(4), true);
    assert_eq!(Solution::is_three(9), true);
    assert_eq!(Solution::is_three(27), false);
}