# Testing in WASIX

Testing in WASIX generall works the same as [testing in
Rust](https://doc.rust-lang.org/book/ch11-01-writing-tests.html), but there's an
important caveat about failing tests in WASIX.

The `wasm64-wasix` target for Rust is effectively a `panic=abort` target which
has no support for unwinding. Most tests report failure, however by panicking!
This means that a failing test will actually abort the whole wasix process, which
isn't always a great experience.

To compound the problems here Rust's test framework by default captures all
output of a panic to print later after all tests have finished executing. If the
process aborts on a panic though, nothing ends up getting printed! Instead
you'll see something like:

```
$ cargo wasix test
...
     Running `/code/wasix-hello-world/target/wasm64-wasix/debug/deps/foo-38c031b0dc9ed5bc.wasm`

running 1 test
test foo ... error: failed to process main module `/code/wasix-hello-world/target/wasm64-wasix/debug/deps/foo-38c031b0dc9ed5bc.wasm`
    caused by: Instantiation error: Trap occurred while invoking start function: wasm trap: unreachable, source location: @4143a
```

and that's not very helpful!

To help with these issues it's recommended to use `--nocapture` which will at
least print *some* information.

```
$ cargo wasix test -- --nocapture
...
     Running `/code/wasix-hello-world/target/wasm64-wasix/debug/deps/foo-38c031b0dc9ed5bc.wasm --nocapture`

running 1 test
test foo ... thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', tests/foo.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: failed to process main module `/code/wasix-hello-world/target/wasm64-wasix/debug/deps/foo-38c031b0dc9ed5bc.wasm`
    caused by: Instantiation error: Trap occurred while invoking start function: wasm trap: unreachable, source location: @4143a
```

In general testing and wasix isn't great today. It's something we hope to improve
over time!
