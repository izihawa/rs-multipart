[![Workflow Status](https://github.com/ferristseng/rust-multipart-rfc7578/workflows/Rust/badge.svg)](https://github.com/ferristseng/rust-multipart-rfc7578/actions?query=workflow%3A%22Rust%22)
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

## Rust Multipart (RFC 7578)

### Components

| Name   | Documentation                                  | Crate                                             |
| ------ | -----------------------------------------------| ------------------------------------------------- |
| common | [![Docs][common docs badge]][common docs link] | [![Crate][common crate badge]][common crate link] |
| hyper  | [![Docs][hyper docs badge]][hyper docs link]   | [![Crate][hyper crate badge]][hyper crate link]   |

This crate contains an implementation of the multipart/form-data media
type described in [RFC 7578](https://tools.ietf.org/html/rfc7578).

### Usage

Add either the Hyper implementation or the Actix implementation to
your Cargo.toml file:

#### Hyper:

```toml
[dependencies]
izihawa-hyper-multipart = "1.0"
```

and import:

```rust
extern crate izihawa_hyper_multipart as multipart;
```


## Note on Server Implementation

I don't have any plans on implementing the server-side of this any time soon. I ended up implementing the client-side because I couldn't find any good libraries that were compatible with hyper >= 0.11.

Please feel free to submit a pull request, I would gladly review it!

## Alternatives

  * [abonander/multipart](https://github.com/abonander/multipart)
  * [abonander/multipart-async](https://crates.io/crates/multipart-async)
  * [mikedilger/formdata](https://github.com/mikedilger/formdata)
  * [jeizsm/rust-multipart-rfc7578](https://github.com/jeizsm/rust-multipart-rfc7578)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[common docs badge]: https://docs.rs/izihawa-common-multipart/badge.svg "izihawa-common-multipart documentation"
[common docs link]: https://docs.rs/izihawa-common-multipart
[common crate badge]: https://img.shields.io/crates/v/izihawa-common-multipart.svg "izihawa-common-multipart crates.io"
[common crate link]: https://crates.io/crates/izihawa-common-multipart
[hyper docs badge]: https://docs.rs/izihawa-hyper-multipart/badge.svg "izihawa-hyper-multipart documentation"
[hyper docs link]: https://docs.rs/izihawa-hyper-multipart
[hyper crate badge]: https://img.shields.io/crates/v/izihawa-hyper-multipart.svg "izihawa-hyper-multipart crates.io"
[hyper crate link]: https://crates.io/crates/izihawa-hyper-multipart
