// use bluesky::state::AppState;
// use bluesky::types::requests::*;
// use bluesky::types::responses::*;
// use std::fs::File;
// use std::sync::{Arc, RwLock};
// use axum::{
//     Router,
//     http::StatusCode,
//     routing::{get, post},
//     extract::{State, Json},
// };
// use axum_test:: TestServer;
// use serde_json::json;

// type MockSharedAppState = Arc<RwLock<AppState>>;
// const MOCK_STATE: &str = "./mocks/app_state.json";

// fn load_fixture_state(path: &str) -> AppState {
//     let file = File::open(path).unwrap();
//     serde_json::from_reader(file).unwrap()
// }

// async fn mock_follow_handler(
//     State(logic): State<MockSharedAppState>,
//     Json(payload): Json<FollowRequest>,
// )-> (StatusCode, Json<FollowResponse>)
// {
//     let mut twitter = logic.write().unwrap();
//     twitter.follow(payload.followee, payload.follower);
//     drop(twitter);
//     (
//         StatusCode::OK,
//         Json(FollowResponse{
//                 msg: format!("Successfully followed")
//         })
//     )

// }
// fn mock_state() -> Router {
//     let mock_state = load_fixture_state(MOCK_STATE);
//     Router::new()
//         .route("/follow", post(mock_follow_handler))
//         .with_state(
//                 Arc::new(RwLock::new(mock_state))
//         )
// }

// #[tokio::test]
// async fn test_unfollow_presaved_state() {
//     let router = mock_state();
//     let server = TestServer::new(router).unwrap();
//     let res = server.post("/follow")
//         .json(&json!({
//             "followee": 1,
//             "follower": 2,
//         }))
//     .await;
//     println!("Result {:?}", res);

//     assert_eq!(res.status_code(), StatusCode::OK);
// }
