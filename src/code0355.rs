#![allow(dead_code)]

// 355. Design Twitter
// https://leetcode.com/problems/design-twitter/
// https://leetcode.cn/problems/design-twitter/
//
// Design a simplified version of Twitter where users can post tweets, follow/unfollow another user,
// and is able to see the 10 most recent tweets in the user's news feed.
//
// Implement the Twitter class:
//
// - Twitter() Initializes your twitter object.
// - void postTweet(int userId, int tweetId) Composes a new tweet with ID tweetId by the user userId.
//   Each call to this function will be made with a unique tweetId.
// - List<Integer> getNewsFeed(int userId) Retrieves the 10 most recent tweet IDs in the user's news feed.
//   Each item in the news feed must be posted by users who the user followed or by the user themself.
//   Tweets must be ordered from most recent to least recent.
// - void follow(int followerId, int followeeId) The user with ID followerId started following the user with ID followeeId.
// - void unfollow(int followerId, int followeeId) The user with ID followerId started unfollowing the user with ID followeeId.
//
// Example 1:
//
// Input
// ["Twitter", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
// [[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]]
// Output
// [null, null, [5], null, null, [6, 5], null, [5]]
//
// Explanation
// Twitter twitter = new Twitter();
// twitter.postTweet(1, 5); // User 1 posts a new tweet (id = 5).
// twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
// twitter.follow(1, 2);    // User 1 follows user 2.
// twitter.postTweet(2, 6); // User 2 posts a new tweet (id = 6).
// twitter.getNewsFeed(1);  // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
// twitter.unfollow(1, 2);  // User 1 unfollows user 2.
// twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
//
// Constraints:
//
// - 1 <= userId, followerId, followeeId <= 500
// - 0 <= tweetId <= 10^4
// - All the tweets have unique IDs.
// - At most 3 * 10^4 calls will be made to postTweet, getNewsFeed, follow, and unfollow.
//

struct Twitter {
    posts: Vec<(i32, i32)>,
    follows: std::collections::HashMap<i32, std::collections::HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            posts: vec![],
            follows: std::collections::HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.posts.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = vec![];
        let mut count = 0;
        for (user, tweet) in self.posts.iter().rev() {
            if *user == user_id || self.follows.get(&user_id).map_or(false, |m| m.get(user).is_some()) {
                feed.push(*tweet);
                count += 1;
                if count == 10 {
                    break;
                }
            }
        }
        feed
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_default().insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_default().remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

#[test]
fn test() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
    twitter.unfollow(1, 2);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
}
