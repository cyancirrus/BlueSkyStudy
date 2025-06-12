use crate::state::AppState;
use std::sync::{Arc, Mutex};

pub type SharedAppState = Arc<Mutex<AppState>>;
pub type UserId = usize;
pub type PostId = usize;
pub type UnixTime = usize;
