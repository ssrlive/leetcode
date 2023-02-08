#![allow(dead_code)]

/*

// 1467. Probability of a Two Boxes Having The Same Number of Distinct Balls
// https://leetcode.com/problems/probability-of-a-two-boxes-having-the-same-number-of-distinct-balls/
// https://leetcode.cn/problems/probability-of-a-two-boxes-having-the-same-number-of-distinct-balls/
//
// Hard
//
// Given 2n balls of k distinct colors. You will be given an integer array balls of size k where balls[i] is the number of balls of color i.

All the balls will be shuffled uniformly at random, then we will distribute the first n balls to the first box and the remaining n balls to the other box (Please read the explanation of the second example carefully).

Please note that the two boxes are considered different. For example, if we have two balls of colors a and b, and two boxes [] and (), then the distribution [a] (b) is considered different than the distribution [b] (a) (Please read the explanation of the first example carefully).

Return the probability that the two boxes have the same number of distinct balls. Answers within 10-5 of the actual value will be accepted as correct.

Example 1:

Input: balls = [1,1]
Output: 1.00000
Explanation: Only 2 ways to divide the balls equally:
- A ball of color 1 to box 1 and a ball of color 2 to box 2
- A ball of color 2 to box 1 and a ball of color 1 to box 2
In both ways, the number of distinct colors in each box is equal. The probability is 2/2 = 1

Example 2:

Input: balls = [2,1,1]
Output: 0.66667
Explanation: We have the set of balls [1, 1, 2, 3]
This set of balls will be shuffled randomly and we may have one of the 12 distinct shuffles with equal probability (i.e. 1/12):
[1,1 / 2,3], [1,1 / 3,2], [1,2 / 1,3], [1,2 / 3,1], [1,3 / 1,2], [1,3 / 2,1], [2,1 / 1,3], [2,1 / 3,1], [2,3 / 1,1], [3,1 / 1,2], [3,1 / 2,1], [3,2 / 1,1]
After that, we add the first two balls to the first box and the second two balls to the second box.
We can see that 8 of these 12 possible random distributions have the same number of distinct colors of balls in each box.
Probability is 8/12 = 0.66667

Example 3:

Input: balls = [1,2,1,2]
Output: 0.60000
Explanation: The set of balls is [1, 2, 2, 3, 4, 4]. It is hard to display all the 180 possible random shuffles of this set but it is easy to check that 108 of them will have the same number of distinct colors in each box.
Probability = 108 / 180 = 0.6

Constraints:

    1 <= balls.length <= 8
    1 <= balls[i] <= 6
    sum(balls) is even.
*/

struct Solution;

impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let balls = balls.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let sum = balls.iter().sum::<_>();
        let mut a = vec![0; balls.len()];
        let mut b = vec![0; balls.len()];
        Solution::dfs(&sum, &balls, &mut a, &mut b, 0, 0, 0) / Solution::perm(&balls)
    }

    fn perm(a: &[i64]) -> f64 {
        let mut ans = 1.0;
        let mut j = 1;
        for &item in a.iter() {
            for k in 1..=item {
                ans = ans * j as f64 / k as f64;
                j += 1;
            }
        }
        ans
    }

    fn dfs(sum: &i64, balls: &[i64], a: &mut [i64], b: &mut [i64], i: usize, sa: i64, sb: i64) -> f64 {
        if sa > sum / 2 || sb > sum / 2 {
            return 0.0;
        }
        if i == balls.len() {
            let ca = a.iter().filter(|&&x| x > 0).count();
            let cb = b.iter().filter(|&&x| x > 0).count();
            if ca != cb {
                return 0.0;
            }
            return Solution::perm(a) * Solution::perm(b);
        }
        let mut ans = 0.0;
        for j in 0..=balls[i] {
            a[i] = j;
            b[i] = balls[i] - j;
            ans += Solution::dfs(sum, balls, a, b, i + 1, sa + a[i], sb + b[i]);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 1], 1.0), (vec![2, 1, 1], 0.66667), (vec![1, 2, 1, 2], 0.6)];
    for (balls, expected) in cases {
        assert!((Solution::get_probability(balls) - expected).abs() < 1e-5);
    }
}
