# Challenge Rust Dio 2024

- CRUD [Done]
- Add Users + Bcrypt [Done]
- Validator [Not Done]
- Add JWT Token [Manual - Not Done]
- Add Profile Picture using Image [Not Done]
- Add JWT Token to Redis [Not Done]
- Implement Swagger [Not Done]
- Refactor Code [Not Done]

Package used
```toml
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
sqlx = { version = "0.8", features = [ "mysql","runtime-tokio", "tls-native-tls","chrono" ] }
dotenvy = "0.15.7"
chrono = {version = "0.4.39", features = ["serde"]}
bcrypt = "0.16.0"
```