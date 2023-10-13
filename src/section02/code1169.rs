#![allow(dead_code)]

// 1169. Invalid Transactions
// https://leetcode.com/problems/invalid-transactions/
// https://leetcode.cn/problems/invalid-transactions/
//
// A transaction is possibly invalid if:
//
// the amount exceeds $1000, or;
// if it occurs within (and including) 60 minutes of another transaction with the same name in a different city.
// You are given an array of strings transaction where transactions[i] consists of comma-separated
// values representing the name, time (in minutes), amount, and city of the transaction.
//
// Return a list of transactions that are possibly invalid. You may return the answer in any order.
//
// Example 1:
//
// Input: transactions = ["alice,20,800,mtv","alice,50,100,beijing"]
// Output: ["alice,20,800,mtv","alice,50,100,beijing"]
// Explanation: The first transaction is invalid because the second transaction occurs within a difference of 60 minutes,
// have the same name and is in a different city. Similarly the second one is invalid too.
//
// Example 2:
//
// Input: transactions = ["alice,20,800,mtv","alice,50,1200,mtv"]
// Output: ["alice,50,1200,mtv"]
//
// Example 3:
//
// Input: transactions = ["alice,20,800,mtv","bob,50,1200,mtv"]
// Output: ["bob,50,1200,mtv"]
//
// Constraints:
//
// - transactions.length <= 1000
// - Each transactions[i] takes the form "{name},{time},{amount},{city}"
// - Each {name} and {city} consist of lowercase English letters, and have lengths between 1 and 10.
// - Each {time} consist of digits, and represent an integer between 0 and 1000.
// - Each {amount} consist of digits, and represent an integer between 0 and 2000.
//

struct Solution;

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        fn helper(transaction: &str) -> (String, String, i32, i32) {
            let mut name = String::new();
            let mut city = String::new();
            let mut amt = 0;
            let mut time = 0;
            let mut a = false;
            let b = false;
            let mut cc = false;
            let mut d = false;
            let transaction = transaction.chars().collect::<Vec<char>>();
            let mut index = 0;
            loop {
                if index >= transaction.len() {
                    break;
                }
                if !a {
                    name.push(transaction[index]);
                    if transaction[index + 1] == ',' {
                        index += 1;
                        a = true;
                    }
                } else if !d {
                    time = time * 10 + transaction[index] as i32 - '0' as i32;
                    if transaction[index + 1] == ',' {
                        index += 1;
                        d = true;
                    }
                } else if !cc {
                    amt = amt * 10 + transaction[index] as i32 - '0' as i32;
                    if transaction[index + 1] == ',' {
                        index += 1;
                        cc = true;
                    }
                } else if !b {
                    city.push(transaction[index]);
                }
                index += 1;
            }
            (name, city, amt, time)
        }

        let mut transactions = transactions;
        transactions.sort();
        let mut ans = Vec::new();
        let mut vis = vec![false; transactions.len()];
        for i in 0..transactions.len() {
            let t1 = helper(&transactions[i]);
            if t1.2 >= 1000 {
                vis[i] = true;
            }
            for j in i + 1..transactions.len() {
                let t2 = helper(&transactions[j]);
                if t1.0 == t2.0 && (t2.3 - t1.3).abs() <= 60 && t1.1 != t2.1 {
                    vis[i] = true;
                    vis[j] = true;
                }
            }
        }
        for i in 0..transactions.len() {
            if vis[i] {
                ans.push(transactions[i].clone());
            }
        }
        ans
    }
}

#[test]
fn test() {
    let transactions = ["alice,20,800,mtv", "alice,50,100,beijing"];
    let transactions = transactions.iter().map(|s| s.to_string()).collect();
    let res = vec!["alice,20,800,mtv", "alice,50,100,beijing"];
    assert_eq!(Solution::invalid_transactions(transactions), res);

    let transactions = ["alice,20,800,mtv", "alice,50,1200,mtv"];
    let transactions = transactions.iter().map(|s| s.to_string()).collect();
    let res = vec!["alice,50,1200,mtv"];
    assert_eq!(Solution::invalid_transactions(transactions), res);

    let transactions = ["alice,20,800,mtv", "bob,50,1200,mtv"];
    let transactions = transactions.iter().map(|s| s.to_string()).collect();
    let res = vec!["bob,50,1200,mtv"];
    assert_eq!(Solution::invalid_transactions(transactions), res);
}
