use bluesky::state::{AppState, Post};

#[test]
fn events() {
    let mut app = AppState::new();
    assert_eq!(app.publish(1, 100), ()); // Post by user 1
    assert_eq!(app.publish(2, 100), ()); // Post by user 2
    assert_eq!(app.news_feed(1), vec![]); // NewsFeed for user 1
    assert_eq!(app.follow(2, 1), ()); // user 2 follows 1
    assert_eq!(app.follow(2, 3), ()); // user 2 follows 3
    assert_eq!(app.publish(3, 300), ()); // Post by user 3
    assert_eq!(app.follow(1, 3), ()); // user 1 follows 3
    assert_eq!(app.news_feed(1), vec![Post::new( 3, 300, 2 )]); // NewsFeed for user 1
    assert_eq!(app.news_feed(2), vec![Post::new(3, 300, 2 ), Post::new(1, 100, 0 )]); // NewsFeed for user 2
    assert_eq!(app.unfollow(2, 1), ()); // user 2 unfollows 1
    assert_eq!(app.news_feed(2), vec![Post::new(3, 300, 2 )]); // NewsFeed for user 2
    assert_eq!(app.unfollow(2, 3), ()); // user 2 unfollows 3
    assert_eq!(app.news_feed(2), vec![]); // NewsFeed for user 2
}
