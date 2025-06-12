use bluesky::state::{AppState, Post};
use chrono::Utc;

fn post_equality(actual: &[Post], expected: &[Post]) -> bool {
    actual
        .iter()
        .map(|p| (p.user_id, p.post_id))
        .collect::<Vec<_>>()
        == expected
            .iter()
            .map(|p| (p.user_id, p.post_id))
            .collect::<Vec<_>>()
}

#[tokio::test]
async fn events() {
    let app = AppState::new();
    assert_eq!(app.publish(1, 100).await, ()); // Post by user 1
    assert_eq!(app.publish(2, 100).await, ()); // Post by user 2
    assert_eq!(app.news_feed(1).await, vec![]); // NewsFeed for user 1
    assert_eq!(app.follow(2, 1).await, ()); // user 2 follows 1
    assert_eq!(app.follow(2, 3).await, ()); // user 2 follows 3
    assert_eq!(app.publish(3, 300).await, ()); // Post by user 3
    assert_eq!(app.follow(1, 3).await, ()); // user 1 follows 3
    assert!(post_equality(
        &app.news_feed(1).await,
        &vec![Post::new(3, 300, Utc::now())]
    )); // NewsFeed for user 1
    assert!(post_equality(
        // NewsFeed for user 2
        &app.news_feed(2).await,
        &vec![Post::new(3, 300, Utc::now()), Post::new(1, 100, Utc::now())]
    ));
    assert_eq!(app.unfollow(2, 1).await, ()); // user 2 unfollows 1
    assert!(post_equality(
        // NewsFeed for user 2
        &app.news_feed(2).await,
        &vec![Post::new(3, 300, Utc::now())]
    ));
    assert_eq!(app.unfollow(2, 3).await, ()); // user 2 unfollows 3
    assert_eq!(app.news_feed(2).await, vec![]); // NewsFeed for user 2
}
