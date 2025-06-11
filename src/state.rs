#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;
use serde::{
    Serialize,
    Deserialize,
};
use crate::types::aliases::{
    TweetId,
    UserId,
    UnixTime,
};

const N:usize =  50;

#[derive(Default, Clone)]
struct Recent {
    history: VecDeque<Tweet>,
}

impl Recent {
    fn new() -> Self {
        Recent { history: VecDeque::with_capacity(N) }
    }
    fn push(&mut self, user_id:UserId, tweet_id:TweetId, time:UnixTime) {
        if self.history.len() >= N {
            self.history.pop_back();
        }
        self.history.push_front(Tweet::new(user_id, tweet_id, time));
    }
    fn pop(&mut self) -> Option<Tweet> {
        self.history.pop_front()
    }
}

pub struct Twitter {
    follows: HashMap<UserId, HashSet<UserId>>,
    tweets: HashMap<UserId, Recent>,
    time: UnixTime, 
}

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Tweet {
    user_id:UserId,
    tweet_id:TweetId,
    time:UnixTime,
}

impl Tweet {
    pub fn new(user_id:UserId, tweet_id:TweetId, time:UnixTime) -> Self {
        Self { user_id, tweet_id, time }
    }
}

impl Ord for Tweet {
    fn cmp(&self, other:&Self) -> Ordering {
        // default cmp is self > other
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Clone)]
pub enum TwitApi {
    Post,
    Follow,
    Unfollow,
    NewsFeed,
}

impl Twitter {
    pub fn new() -> Self {
        // UnixTime here just for simple prototyping will be replaced with calls
        Self { follows:HashMap::new(), tweets:HashMap::new(), time:0 }
    }
    pub fn publish(&mut self, user_id:UserId, tweet_id:TweetId) {
        self.tweets.entry(user_id).or_default().push(user_id, tweet_id, self.time);
        self.time+=1;

    }
    pub fn follow(&mut self, follower_id:UserId, followee_id:UserId) {
        self.follows.entry(follower_id).or_default().insert(followee_id);
    }
    pub fn unfollow(&mut self, follower_id:UserId, followee_id:UserId) {
        self.follows.entry(follower_id).or_default().remove(&followee_id);
    }
    pub fn news_feed(&mut self, user_id:UserId) -> Vec<Tweet> {
        let mut news: HashMap<UserId, Recent> = HashMap::new();
        if let Some(subscribes) = self.follows.get(&user_id) {
            for ldr in subscribes {
                if let Some(twts) = self.tweets.get(ldr) {
                    news.insert(*ldr, twts.clone());
                }
            }
        }
        self._nf_merge_k_(10, &mut news)
    }

    fn _nf_merge_k_(&mut self, k:usize, subscribes:&mut HashMap<UserId, Recent>) -> Vec<Tweet> {
        let mut sorted:BinaryHeap<Tweet>= BinaryHeap::new();
        let mut recents:Vec<Tweet> = Vec::with_capacity(k);

        for history in subscribes.values_mut() {
            if let Some(t) = history.pop() {
                sorted.push(Tweet::new(t.user_id, t.tweet_id, t.time));
            }
        }
        // prototype -- could have number of availables and decrement to break early
        for _ in 0..k {
            if let Some(tweet) =  sorted.pop() {
                if let Some(unseen)  = subscribes.get_mut(&tweet.user_id) {
                    if let Some(r) = unseen.pop() {
                        sorted.push(Tweet::new(r.user_id, r.tweet_id, r.time));
                    }
                }
                recents.push(tweet)
            }
        }
        recents
    }
}
