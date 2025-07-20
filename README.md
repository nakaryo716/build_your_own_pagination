# Build Your Own Pagination In Rust
## Overview
Create `Offset-Based` and `Cursor-Based` pagination in Rust language.  
We use `hyper` and `http2` feature.  

## Run
Run database(postgres)
```
docker compose up
```
Install sqlx-cli
```
cargo install sqlx-cli
```
Run migration
```
sqlx migrate run
```
Run application
```
cargo run
```
