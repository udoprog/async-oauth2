# async-oauth2

<a href="https://crates.io/crates/async-oauth2"><img src="https://img.shields.io/crates/v/async-oauth2.svg"></a>
<a href="https://travis-ci.org/udoprog/async-oauth2-rs"><img src="https://travis-ci.org/udoprog/async-oauth2-rs.svg?branch=master"></a>

An asynchronous first implementation of OAuth2 for Rust.

This is a fork of [`oauth2-rs`](https://github.com/ramosbugs/oauth2-rs).

The main differences are:
* Removed unecessary type parameters on Client ([see discussion here]).
* Only support one client implementation (reqwest).
* Remove most newtypes except `Scope` and the secret ones since they made the API harder to use.

[see discussion here]: https://github.com/ramosbugs/oauth2-rs/issues/44#issuecomment-50158653

Documentation is available on [docs.rs](https://docs.rs/crate/async-oauth2) or check the [examples](https://github.com/udoprog/async-oauth2/tree/master/examples).

## Examples

If you want to run some of our examples, you need to register an application that has a redirect URL of `http://localhost:8080/api/auth/redirect`, then you can run the clients like this:

```
cargo run --example spotify --client-id <client-id> --client-secret <client-secret>
cargo run --example google --client-id <client-id> --client-secret <client-secret>
cargo run --example twitch --client-id <client-id> --client-secret <client-secret>
```