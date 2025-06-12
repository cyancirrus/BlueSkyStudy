use std::sync::Arc;
use bluesky::state::AppState;
use futures::future::join_all;

#[tokio::test]
async fn concurrent_inserts() {
    let app = Arc::new(AppState::new());
    let user_count = 1000;
    let post_count = 1000;

    // Everyone follows 10 random users
    for user_id in 1..=user_count {
        for offset in 1..=10 {
            let followee_id = ((user_id + offset - 1) % user_count) + 1;
            if user_id != followee_id {
                let app_clone = Arc::clone(&app);
                tokio::spawn(async move {
                    app_clone.follow(user_id, followee_id).await;
                });
            }
        }
    }

    // Post concurrently
    let mut tasks = vec![];
    for i in 0..post_count {
        let app_clone = Arc::clone(&app);
        let user_id = (i % user_count) + 1;
        let post_id = i as u64;
        tasks.push(tokio::spawn(async move {
            app_clone.publish(user_id, post_id as usize).await;
        }));
    }

    join_all(tasks).await;

    // Check feeds
    let mut feed_tasks = vec![];
    for user_id in 1..=10 {
        let app_clone = Arc::clone(&app);
        feed_tasks.push(tokio::spawn(async move {
            let feed = app_clone.news_feed(user_id).await;
            println!("User {}'s feed has {} posts", user_id, feed.len());
        }));
    }

    futures::future::join_all(feed_tasks).await;
}

