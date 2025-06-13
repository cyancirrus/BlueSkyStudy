# A BlueSky-like Async Study

A minimal async message board / social feed engine written in Rust.  
This project explores high-concurrency design using `DashMap`, per-user rotating buffers, and non-blocking async logic.  

---

## Features

✅ Async-safe follow, unfollow, publish, and newsfeed operations  
✅ Per-user recent post buffers (rotating, capped size)  
✅ Efficient merge-K logic for news feeds  
✅ Scales to thousands of users / posts with low contention  
✅ Clean separation of logic + interface (ready for HTTP API)

---

## Future Goals

✨ Add SQLite persistence:  
- Dump older post history (beyond in-memory buffer) to DB  
- Load from DB to replenish buffer if needed  

✨ Extend tests for large-scale simulation  

✨ Add API server with Axum or Actix

---

## Quick Start

```bash
cargo run
```

Run tests:
```bash
cargo test
```

Design Notes

- Uses DashMap for fast concurrent reads/writes per user
- Rotating buffer (e.g. VecDeque) keeps memory bounded
- No single lock bottleneck; follows/posts/newsfeed all per-user granular
- Easy to extend with storage layers or API interface

Example Feed Output

```
User 1's feed has 10 posts
User 2's feed has 10 posts
```

License
MIT
