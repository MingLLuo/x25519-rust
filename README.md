# X25519 Key Exchange

- Implemented by Rust, using the `crypto-bigint` crate for big integer arithmetic.
- For the `cswap` part, I used the provided method from the crate above.
- Most of the code contains a test suites, which is from the test vectors / `monty_test` provided, with `CodeX`'s help.
- Might improve performance by `#[inline]` or change the way we manipulate the FieldElements, but I think it's good enough for now.
