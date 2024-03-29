#![allow(dead_code)]

// 929. Unique Email Addresses
// https://leetcode.com/problems/unique-email-addresses/
// https://leetcode.cn/problems/unique-email-addresses/
//
// Every valid email consists of a local name and a domain name, separated by the '@' sign. Besides lowercase letters, the email may contain one or more '.' or '+'.
//
// For example, in "alice@leetcode.com", "alice" is the local name, and "leetcode.com" is the domain name.
//
// If you add periods '.' between some characters in the local name part of an email address, mail sent there will
// be forwarded to the same address without dots in the local name. Note that this rule does not apply to domain names.
//
// For example, "alice.z@leetcode.com" and "alicez@leetcode.com" forward to the same email address.
//
// If you add a plus '+' in the local name, everything after the first plus sign will be ignored.
// This allows certain emails to be filtered. Note that this rule does not apply to domain names.
//
// For example, "m.y+name@email.com" will be forwarded to "my@email.com".
// It is possible to use both of these rules at the same time.
//
// Given an array of strings emails where we send one email to each emails[i], return the number of different addresses that actually receive mails.
//
// Example 1:
//
// Input: emails = ["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]
// Output: 2
// Explanation: "testemail@leetcode.com" and "testemail@lee.tcode.com" actually receive mails.
//
// Example 2:
//
// Input: emails = ["a@leetcode.com","b@leetcode.com","c@leetcode.com"]
// Output: 3
//
// Constraints:
//
// - 1 <= emails.length <= 100
// - 1 <= emails[i].length <= 100
// - emails[i] consist of lowercase English letters, '+', '.' and '@'.
// - Each emails[i] contains exactly one '@' character.
// - All local and domain names are non-empty.
// - Local names do not start with a '+' character.
// - Domain names end with the ".com" suffix.
//

struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut n = 0;
        let set = &mut std::collections::HashSet::new();
        for s in &emails {
            let mut it = s.splitn(2, '@');
            if let (Some(loc), Some(dom)) = (it.next(), it.next()) {
                let a: String = loc.chars().take_while(|&c| c != '+').filter(|&c| c != '.').collect();
                n += set.insert((a, dom)) as i32;
            }
        }
        n
    }
}

#[test]
fn test() {
    let emails = [
        "test.email+alex@leetcode.com",
        "test.e.mail+bob.cathy@leetcode.com",
        "testemail+david@lee.tcode.com",
    ];
    let emails = emails.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::num_unique_emails(emails), 2);

    let emails = ["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"];
    let emails = emails.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::num_unique_emails(emails), 3);
}
