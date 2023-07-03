#![allow(dead_code)]

// 721. Accounts Merge
// https://leetcode.com/problems/accounts-merge/
// https://leetcode.cn/problems/accounts-merge/
//
// Given a list of accounts where each element accounts[i] is a list of strings, where the first element accounts[i][0] is a name,
// and the rest of the elements are emails representing emails of the account.
//
// Now, we would like to merge these accounts. Two accounts definitely belong to the same person if there is some common email
// to both accounts. Note that even if two accounts have the same name, they may belong to different people as people could have
// the same name. A person can have any number of accounts initially, but all of their accounts definitely have the same name.
//
// After merging the accounts, return the accounts in the following format: the first element of each account is the name,
// and the rest of the elements are emails in sorted order. The accounts themselves can be returned in any order.
//
// Example 1:
//
// Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],
//        ["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
// Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
// Explanation:
// The first and second John's are the same person as they have the common email "johnsmith@mail.com".
// The third John and Mary are different people as none of their email addresses are used by other accounts.
// We could return these lists in any order, for example the answer [['Mary', 'mary@mail.com'], ['John', 'johnnybravo@mail.com'],
// ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']] would still be accepted.
//
// Example 2:
//
// Input: accounts = [["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],
//        ["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],
//        ["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]]
// Output: [["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],
//        ["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],
//        ["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
//
// Constraints:
//
// - 1 <= accounts.length <= 1000
// - 2 <= accounts[i].length <= 10
// - 1 <= accounts[i][j].length <= 30
// - accounts[i][0] consists of English letters.
// - accounts[i][j] (for j > 0) is a valid email.
//

struct Solution;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        fn _accounts_merge(accounts: &[Vec<String>]) -> Option<Vec<Vec<&String>>> {
            use std::collections::{HashMap, HashSet};
            let mut email_to_name = HashMap::new();
            let mut graph = HashMap::<_, HashSet<_>>::new();

            for account in accounts.iter() {
                let name = account.get(0)?;
                let emails = &account[1..];

                for email in emails.iter() {
                    email_to_name.insert(email, name);
                    graph.entry(email).or_default().insert(email);
                }

                for i in 0..emails.len() {
                    for j in i + 1..emails.len() {
                        let email_i = emails.get(i)?;
                        let email_j = emails.get(j)?;
                        graph.entry(email_i).or_default().insert(email_j);
                        graph.entry(email_j).or_default().insert(email_i);
                    }
                }
            }

            let mut visited = HashSet::new();
            let mut result = Vec::new();

            for &email in graph.keys() {
                if !visited.contains(email) {
                    let mut stack = vec![email];
                    let mut component = vec![];

                    while let Some(email) = stack.pop() {
                        if !visited.contains(email) {
                            visited.insert(email);
                            component.push(email);
                            stack.extend(graph.get(email)?);
                        }
                    }

                    component.sort();
                    let mut account = vec![*email_to_name.get(*component.get(0)?)?];
                    account.extend(component.iter());
                    result.push(account);
                }
            }
            Some(result)
        }
        let result = _accounts_merge(&accounts).unwrap_or_default();
        result.iter().map(|v| v.iter().map(|&s| s.clone()).collect()).collect()
    }

    // Union Find Solution
    pub fn accounts_merge2(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        fn find<'a>(fa: &mut HashMap<&'a String, &'a String>, x: &'a String) -> &'a String {
            if x == fa[x] {
                return x;
            }
            let fd = find(fa, fa[x]);
            fa.insert(x, fd);
            fa[x]
        }

        fn union<'a>(fa: &mut HashMap<&'a String, &'a String>, x: &'a String, y: &'a String) {
            let a = find(fa, x);
            let b = find(fa, y);
            fa.insert(b, a);
        }

        fn _accounts_merge2(accounts: &[Vec<String>]) -> Option<Vec<Vec<&String>>> {
            let mut fa = HashMap::new();
            let mut email_name = HashMap::new();
            for acc in accounts {
                let name = &acc[0];
                for item in acc.iter().skip(1) {
                    fa.insert(item, item);
                    email_name.entry(item).or_insert(name);
                }
            }
            for acc in accounts {
                for i in 2..acc.len() {
                    union(&mut fa, &acc[1], &acc[i]);
                }
            }
            let mut name_emails = HashMap::<_, Vec<_>>::new();
            for em in email_name.keys() {
                name_emails.entry(find(&mut fa, em)).or_default().push(*em);
            }
            for v in name_emails.values_mut() {
                v.sort();
            }

            let mut res = Vec::new();
            for fa_email in name_emails.keys() {
                let mut data = Vec::new();
                data.push(*email_name.get(*fa_email)?);
                for t in name_emails.get(*fa_email)?.iter() {
                    data.push(t);
                }
                res.push(data);
            }
            Some(res)
        }
        let result = _accounts_merge2(&accounts).unwrap_or_default();
        result.iter().map(|v| v.iter().map(|&s| s.clone()).collect()).collect()
    }
}

#[test]
fn test() {
    let accounts = vec![
        vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
        vec!["John", "johnsmith@mail.com", "john00@mail.com"],
        vec!["Mary", "mary@mail.com"],
        vec!["John", "johnnybravo@mail.com"],
    ];
    let accounts = accounts.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();
    let mut result = Solution::accounts_merge2(accounts);
    result.sort();
    let expected = vec![
        vec!["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"],
        vec!["John", "johnnybravo@mail.com"],
        vec!["Mary", "mary@mail.com"],
    ];
    assert_eq!(result, expected);

    let accounts = vec![
        vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
        vec!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
        vec!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
        vec!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
        vec!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
    ];
    let accounts = accounts.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();
    let mut result = Solution::accounts_merge2(accounts);
    result.sort();
    let expected = vec![
        vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
        vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
        vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
        vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
        vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
    ];
    assert_eq!(result, expected);
}
