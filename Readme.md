# Minigrep

Rust implementation of the common linux utility grep. Based on [this tutorial](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)

### Testing

cargo test

### Running

```rust
cargo run The poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep The poem.txt`
Then there's a pair of us - don't tell!
They'd banish us, you know.
```

```rust
IGNORE_CASE=true cargo run The poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep The poem.txt`
Then there's a pair of us - don't tell!
They'd banish us, you know.
To tell your name the livelong day
```
