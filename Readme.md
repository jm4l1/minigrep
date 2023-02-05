# Minigrep

Rust implementation of the common linux utility grep. Based on [this tutorial](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)

### Testing

```rust
cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/minigrep-02b68e605ba70fc4)

running 3 tests
test tests::one_result ... ok
test tests::search_insensitive ... ok
test tests::search_sensitive ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-79623330729acc18)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

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
