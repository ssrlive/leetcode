#![allow(dead_code)]

/*

// 2591. Distribute Money to Maximum Children
// https://leetcode.com/problems/distribute-money-to-maximum-children/
// https://leetcode.cn/problems/distribute-money-to-maximum-children/
//
// Easy
//
// You are given an integer money denoting the amount of money (in dollars) that you have and another integer children denoting the number of children that you must distribute the money to.

You have to distribute the money according to the following rules:

    All money must be distributed.
    Everyone must receive at least 1 dollar.
    Nobody receives 4 dollars.

Return the maximum number of children who may receive exactly 8 dollars if you distribute the money according to the aforementioned rules. If there is no way to distribute the money, return -1.

Example 1:

Input: money = 20, children = 3
Output: 1
Explanation:
The maximum number of children with 8 dollars will be 1. One of the ways to distribute the money is:
- 8 dollars to the first child.
- 9 dollars to the second child.
- 3 dollars to the third child.
It can be proven that no distribution exists such that number of children getting 8 dollars is greater than 1.

Example 2:

Input: money = 16, children = 2
Output: 2
Explanation: Each child can be given 8 dollars.

Constraints:

    1 <= money <= 200
    2 <= children <= 30
*/

struct Solution;

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        let pool = money - children;
        if pool < 0 {
            return -1;
        }
        if pool / 7 == children && pool % 7 == 0 {
            return children;
        }
        if pool / 7 == children - 1 && pool % 7 == 3 {
            return children - 2;
        }
        (children - 1).min(pool / 7)
    }

    pub fn dist_money2(money: i32, children: i32) -> i32 {
        pub fn _dist_money(money: u32, children: u32, desired_amount: u32, bad_amount: u32) -> Result<u32, ()> {
            assert!(children >= 2);
            assert!(desired_amount > bad_amount);
            assert!(desired_amount > 2);
            assert!(bad_amount > 2);

            let mut pool = money;
            pool = pool.checked_sub(children).ok_or(())?;

            let mut num_children_with_desired_amount = 0;
            while pool >= desired_amount - 1 && num_children_with_desired_amount < children {
                pool -= desired_amount - 1;
                num_children_with_desired_amount += 1;
            }

            let num_children_without_desired_amount = children - num_children_with_desired_amount;
            let num_children_with_desired_amount_new = match num_children_without_desired_amount {
                0 => {
                    if pool > 0 {
                        num_children_with_desired_amount - 1
                    } else {
                        num_children_with_desired_amount
                    }
                }
                1 => {
                    if pool == bad_amount - 1 {
                        assert!(num_children_with_desired_amount > 0);
                        num_children_with_desired_amount - 1
                    } else {
                        num_children_with_desired_amount
                    }
                }
                _ => num_children_with_desired_amount,
            };
            Ok(num_children_with_desired_amount_new)
        }

        _dist_money(money as u32, children as u32, 8, 4)
            .map(|x| x as i32)
            .unwrap_or(-1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::dist_money(20, 3), 1);
    assert_eq!(Solution::dist_money(16, 2), 2);
    assert_eq!(Solution::dist_money(1, 2), -1);
}
