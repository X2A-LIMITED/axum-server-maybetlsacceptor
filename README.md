# axum-server-maybtlsacceptor

axum-server-maybetlsacceptor is an [axum-server](https://github.com/programatik29/axum-server)-compatible enum that lets you easily accepts connections with or without TLS.
The main goals were:

- no duplication of routes, layers or other serving code to add the acceptor
- no dynamic dispatch

[pin-project](https://github.com/taiki-e/pin-project) was chosen instead of [pin-project-lite](https://github.com/taiki-e/pin-project-lite) because it does not play well with other attributes such as `cfg`.

## Features

By default, no feature is enabled, meaning no TLS backend is available.

- `rustls`: enable the rustls TLS backend
- `openssl`: enable the openssl TLS backend

## Compatibility

Version `0.7.x` is compatible with `axum-server` version `0.7.y`.

This crate's versioning will try to follow `axum-server` major versioning (or minor while being unstable `0.x.y`).

## Usage Example

You can find a basic example in [examples/basic.rs](examples/basic.rs)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.