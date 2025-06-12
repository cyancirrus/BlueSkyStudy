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
    PostId,
    UserId,
    UnixTime,
};

const N:usize =  50;

#[derive(Default, Clone, Serialize, Deserialize)]
struct Recent {
    history: VecDeque<Post>,
}

impl Recent {
    fn new() -> Self {
        Recent { history: VecDeque::with_capacity(N) }
    }
    fn push(&mut self, user_id:UserId, post_id:PostId, time:UnixTime) {
        if self.history.len() >= N {
            self.history.pop_back();
        }
        self.history.push_front(Post::new(user_id, post_id, time));
    }
    fn pop(&mut self) -> Option<Post> {
        self.history.pop_front()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    follows: HashMap<UserId, HashSet<UserId>>,
    posts: HashMap<UserId, Recent>,
    time: UnixTime, 
}

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    user_id:UserId,
    post_id:PostId,
    time:UnixTime,
}

impl Post {
    pub fn new(user_id:UserId, post_id:PostId, time:UnixTime) -> Self {
        Self { user_id, post_id, time }
    }
}

impl Ord for Post {
    fn cmp(&self, other:&Self) -> Ordering {
        // default cmp is self > other
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub enum BoardApi {
    Post,
    Follow,
    Unfollow,
    NewsFeed,
}

impl AppState {
    pub fn new() -> Self {
        // UnixTime here just for simple prototyping will be replaced with calls
        Self { follows:HashMap::new(), posts:HashMap::new(), time:0 }
    }
    pub fn publish(&mut self, user_id:UserId, post_id:PostId) {
        self.posts.entry(user_id).or_default().push(user_id, post_id, self.time);
        self.time+=1;

    }
    pub fn follow(&mut self, follower_id:UserId, followee_id:UserId) {
        self.follows.entry(follower_id).or_default().insert(followee_id);
    }
    pub fn unfollow(&mut self, follower_id:UserId, followee_id:UserId) {
        self.follows.entry(follower_id).or_default().remove(&followee_id);
    }
    pub fn news_feed(&mut self, user_id:UserId) -> Vec<Post> {
        let mut news: HashMap<UserId, Recent> = HashMap::new();
        if let Some(subscribes) = self.follows.get(&user_id) {
            for ldr in subscribes {
                if let Some(twts) = self.posts.get(ldr) {
                    news.insert(*ldr, twts.clone());
                }
            }
        }
        self._nf_merge_k_(10, &mut news)
    }

    fn _nf_merge_k_(&mut self, k:usize, subscribes:&mut HashMap<UserId, Recent>) -> Vec<Post> {
        let mut sorted:BinaryHeap<Post>= BinaryHeap::new();
        let mut recents:Vec<Post> = Vec::with_capacity(k);

        for history in subscribes.values_mut() {
            if let Some(t) = history.pop() {
                sorted.push(Post::new(t.user_id, t.post_id, t.time));
            }
        }
        // prototype -- could have number of availables and decrement to break early
        for _ in 0..k {
            if let Some(post) =  sorted.pop() {
                if let Some(unseen)  = subscribes.get_mut(&post.user_id) {
                    if let Some(r) = unseen.pop() {
                        sorted.push(Post::new(r.user_id, r.post_id, r.time));
                    }
                }
                recents.push(post)
            }
        }
        recents
    }
}
