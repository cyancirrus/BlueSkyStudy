#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;
use dashmap::DashMap;
use chrono::prelude::{Utc, DateTime};
use tokio::sync::RwLock;
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
pub struct Recent {
    history: VecDeque<Post>,
}

impl Recent {
    fn new() -> Self {
        Recent { history: VecDeque::with_capacity(N) }
    }
    fn push(&mut self, user_id:UserId, post_id:PostId, time:DateTime<Utc>) {
        if self.history.len() >= N {
            self.history.pop_back();
        }
        self.history.push_front(Post { user_id, post_id, time });
    }
    fn pop(&mut self) -> Option<Post> {
        self.history.pop_front()
    }
}


#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub user_id:UserId,
    pub post_id:PostId,
    time:DateTime<Utc>,
}

impl Post {
    pub fn new(user_id:UserId, post_id:PostId, time:DateTime<Utc>) -> Self {
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


// #[derive(Deserialize)]
pub struct AppState {
    follows: DashMap<UserId, RwLock<HashSet<UserId>>>,
    posts: DashMap<UserId, RwLock<Recent>>,
    time: UnixTime, 
}


impl AppState {
    pub fn new() -> Self {
        // UnixTime here just for simple prototyping will be replaced with calls
        Self { follows: DashMap::new(), posts:DashMap::new(), time:0 }
    }
    pub async fn publish(&self, user_id:UserId, post_id:PostId) {
        let history = self.posts.entry(user_id).or_default(); 
        let mut access = history.write().await;
        access.push(user_id, post_id, Utc::now());

    }
    pub async fn follow(&self, follower_id:UserId, followee_id:UserId) {
        let followees = self.follows.entry(follower_id) .or_default();
        let mut access  = followees.write().await;
        access.insert(followee_id);
    }
    pub async fn unfollow(&self, follower_id:UserId, followee_id:UserId) {
        let followees = self.follows.entry(follower_id).or_default();
        let mut access = followees.write().await;
        access.remove(&followee_id);
    }
    pub async fn news_feed(&self, user_id:UserId) -> Vec<Post> {
        let mut news: HashMap<UserId, Recent> = HashMap::new();
        if let Some(subscribes) = self.follows.get(&user_id) {
            let rd_sub_access = subscribes.read().await;
            for ldr in rd_sub_access.iter() {
                if let Some(twts) = self.posts.get(ldr) {
                    let rd_twts_access = twts.read().await;
                    news.insert(*ldr, rd_twts_access.clone());
                }
            }
        }
        self._nf_merge_k_(10, &mut news).await
    }

    pub async fn _nf_merge_k_(&self, k:usize, subscribes:&mut HashMap<UserId, Recent>) -> Vec<Post> {
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
