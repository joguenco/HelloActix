# HelloActix

## Upgrade Rust
```
rustup update
```
## Code format
```
cargo fmt
```
## Run
```
cargo run
```
## Hot Reload
```
cargo install --locked watchexec-cli
watchexec -w src -r cargo run
```
## Test Routes
```
http http://0.0.0.0:8080/ping
```

## Diesel
Create .env file
```
DATABASE_URL=postgres://hello:h@localhost/hello
```

```
diesel setup
```
Create migration
```
diesel migration generate create_greetings
```
Run migration
```
diesel migration run
```
Redo migration
```
diesel migration redo   
```

