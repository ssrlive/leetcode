#![allow(dead_code)]

// 1311. Get Watched Videos by Your Friends
// https://leetcode.com/problems/get-watched-videos-by-your-friends/
// https://leetcode.cn/problems/get-watched-videos-by-your-friends/
//
// Medium
//
// There are n people, each person has a unique id between 0 and n-1. Given the arrays watchedVideos
// and friends, where watchedVideos[i] and friends[i] contain the list of watched videos and the list
// of friends respectively for the person with id = i.
//
// Level 1 of videos are all watched videos by your friends, level 2 of videos are all watched videos
// by the friends of your friends and so on. In general, the level k of videos are all watched videos
// by people with the shortest path exactly equal to k with you. Given your id and the level of videos,
// return the list of videos ordered by their frequencies (increasing). For videos with the same frequency
// order them alphabetically from least to greatest.
//
// Example 1:
//
// Input: watchedVideos = [["A","B"],["C"],["B","C"],["D"]], friends = [[1,2],[0,3],[0,3],[1,2]], id = 0, level = 1
// Output: ["B","C"]
// Explanation:
// You have id = 0 (green color in the figure) and your friends are (yellow color in the figure):
// Person with id = 1 -> watchedVideos = ["C"]
// Person with id = 2 -> watchedVideos = ["B","C"]
// The frequencies of watchedVideos by your friends are:
// B -> 1
// C -> 2
//
// Example 2:
//
// Input: watchedVideos = [["A","B"],["C"],["B","C"],["D"]], friends = [[1,2],[0,3],[0,3],[1,2]], id = 0, level = 2
// Output: ["D"]
// Explanation:
// You have id = 0 (green color in the figure) and the only friend of your friends is the person
// with id = 3 (yellow color in the figure).
//
// Constraints:
//
// -    n == watchedVideos.length == friends.length
// -    2 <= n <= 100
// -    1 <= watchedVideos[i].length <= 100
// -    1 <= watchedVideos[i][j].length <= 8
// -    0 <= friends[i].length < n
// -    0 <= friends[i][j] < n
// -    0 <= id < n
// -    1 <= level < n
// -    if friends[i] contains j, then friends[j] contains i
//

struct Solution;

impl Solution {
    pub fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
        let mut visited = vec![false; friends.len()];
        let mut queue = vec![id as usize];
        visited[id as usize] = true;
        let mut level = level;
        while level > 0 {
            let mut next_queue = Vec::new();
            for i in queue {
                for j in &friends[i] {
                    if !visited[*j as usize] {
                        visited[*j as usize] = true;
                        next_queue.push(*j as usize);
                    }
                }
            }
            queue = next_queue;
            level -= 1;
        }
        let mut freq = std::collections::HashMap::new();
        for i in queue {
            for j in &watched_videos[i] {
                *freq.entry(j).or_insert(0) += 1;
            }
        }
        let mut freq = freq.into_iter().collect::<Vec<_>>();
        freq.sort_by(|a, b| if a.1 == b.1 { a.0.cmp(b.0) } else { a.1.cmp(&b.1) });
        freq.into_iter().map(|x| x.0.to_string()).collect()
    }
}

#[test]
fn test() {
    let watched_videos = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string()],
        vec!["B".to_string(), "C".to_string()],
        vec!["D".to_string()],
    ];
    let friends = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
    let id = 0;
    let level = 1;
    let result = Solution::watched_videos_by_friends(watched_videos, friends, id, level);
    assert_eq!(result, vec!["B".to_string(), "C".to_string()]);

    let watched_videos = vec![
        vec!["A".to_string(), "B".to_string()],
        vec!["C".to_string()],
        vec!["B".to_string(), "C".to_string()],
        vec!["D".to_string()],
    ];
    let friends = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
    let id = 0;
    let level = 2;
    let result = Solution::watched_videos_by_friends(watched_videos, friends, id, level);
    assert_eq!(result, vec!["D".to_string()]);
}
