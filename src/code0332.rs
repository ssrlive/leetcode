#![allow(dead_code)]

// 332. Reconstruct Itinerary
// https://leetcode.com/problems/reconstruct-itinerary/
//
// You are given a list of airline tickets where tickets[i] = [fromi, toi] represent the departure
// and the arrival airports of one flight. Reconstruct the itinerary in order and return it.
//
// All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK".
// If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.
//
// For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
// You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.
//
// Example 1:
//
// Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
// Output: ["JFK","MUC","LHR","SFO","SJC"]
//
// Example 2:
//
// Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
// Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
// Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"] but it is larger in lexical order.
//
// Constraints:
//
// - 1 <= tickets.length <= 300
// - tickets[i].length == 2
// - fromi.length == 3
// - toi.length == 3
// - fromi and toi consist of uppercase English letters.
// - fromi != toi
//

struct Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

        let mut graph: HashMap<&str, BinaryHeap<Reverse<&str>>> = HashMap::new();
        for ticket in tickets.iter() {
            graph
                .entry(&ticket[0])
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(&ticket[1]));
        }
        let mut answer: Vec<String> = Vec::with_capacity(tickets.len() + 1);
        let mut stack: Vec<&str> = vec!["JFK"];
        while let Some(src) = stack.last() {
            if let Some(dsts) = graph.get_mut(src) {
                if !dsts.is_empty() {
                    if let Some(dst) = dsts.pop() {
                        stack.push(dst.0);
                    }
                    continue;
                }
            }
            if let Some(last) = stack.pop() {
                answer.push(last.to_string());
            }
        }
        answer.reverse();
        answer
    }
}

#[test]
fn test() {
    let tickets = vec![
        vec!["MUC".to_string(), "LHR".to_string()],
        vec!["JFK".to_string(), "MUC".to_string()],
        vec!["SFO".to_string(), "SJC".to_string()],
        vec!["LHR".to_string(), "SFO".to_string()],
    ];
    let result = vec![
        "JFK".to_string(),
        "MUC".to_string(),
        "LHR".to_string(),
        "SFO".to_string(),
        "SJC".to_string(),
    ];
    assert_eq!(Solution::find_itinerary(tickets), result);
}
