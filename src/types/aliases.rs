use crate::state::Twitter;
use std::sync::{Arc, Mutex};

pub type SharedTwitter = Arc<Mutex<Twitter>>;
pub type UserId = usize;
pub type TweetId = usize;
pub type UnixTime = usize;
