mod state;
use axum::Router;
// use tower::ServiceBuilder;
use tracing_subscriber;
use state::TwitApi;
use state::*;
// mod routes;

fn main() {
    use TwitApi::*;

    let actions = vec![
        Post,           // user 1 posts
        Post,           // user 2 posts
        NewsFeed,       // user 1 asks for news
        Follow,         // user 2 follows 1
        Follow,         // user 2 follows 3
        Post,           // user 3 posts
        Follow,         // user 1 follows 3
        NewsFeed,       // get news (1)
        NewsFeed,       // get news (2)
        Unfollow,       // user 2 unfollows 1
        NewsFeed,       // get news (2)
    ];

    let params = vec![
        vec![1, 100],    // Post by user 1
        vec![2, 200],    // Post by user 2
        vec![1],         // NewsFeed for user 1
        vec![2, 1],      // user 2 follows 1
        vec![2, 3],      // user 2 follows 3
        vec![3, 300],    // Post by user 3
        vec![1, 3],      // user 1 follows 3
        vec![1],         // NewsFeed for user 1
        vec![2],         // NewsFeed for user 2
        vec![2, 1],      // user 2 unfollows 1
        vec![2],         // NewsFeed for user 2
    ];

    testing_interface(actions, params);
}
