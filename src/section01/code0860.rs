#![allow(dead_code)]

// 860. Lemonade Change
// https://leetcode.com/problems/lemonade-change/
// https://leetcode.cn/problems/lemonade-change/
//
// At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from you and order one at a time (in the order specified by bills).
// Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill.
// You must provide the correct change to each customer so that the net transaction is that the customer pays $5.
//
// Note that you do not have any change in hand at first.
//
// Given an integer array bills where bills[i] is the bill the ith customer pays,
// return true if you can provide every customer with the correct change, or false otherwise.
//
// Example 1:
//
// Input: bills = [5,5,5,10,20]
// Output: true
// Explanation:
// From the first 3 customers, we collect three $5 bills in order.
// From the fourth customer, we collect a $10 bill and give back a $5.
// From the fifth customer, we give a $10 bill and a $5 bill.
// Since all customers got correct change, we output true.
//
// Example 2:
//
// Input: bills = [5,5,10,10,20]
// Output: false
// Explanation:
// From the first two customers in order, we collect two $5 bills.
// For the next two customers in order, we collect a $10 bill and give back a $5 bill.
// For the last customer, we can not give the change of $15 back because we only have two $10 bills.
// Since not every customer received the correct change, the answer is false.
//
// Constraints:
//
// - 1 <= bills.length <= 10^5
// - bills[i] is either 5, 10, or 20.
//

struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut num_5, mut num_10) = (0, 0);
        for &bill in &bills {
            if bill == 5 {
                num_5 += 1;
            } else if bill == 10 {
                match num_5 >= 1 {
                    true => num_5 -= 1,
                    false => return false,
                };
                num_10 += 1;
            } else if num_10 >= 1 && num_5 >= 1 {
                num_10 -= 1;
                num_5 -= 1;
            } else if num_5 >= 3 {
                num_5 -= 3;
            } else {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    assert!(Solution::lemonade_change(vec![5, 5, 10]));
    assert!(!Solution::lemonade_change(vec![10, 10]));
    assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    assert!(!Solution::lemonade_change(vec![5, 5, 5, 5, 20, 20, 5, 5, 20, 5]));
}
