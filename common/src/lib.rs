// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578).
//!
//! ## Usage
//!
//! Add either the Hyper implementation or the Actix implementation to
//! your Cargo.toml file:
//!
//! ### Hyper:
//!
//! ```toml
//! [dependencies]
//! izihawa-hyper-multipart = "1.0"
//! ```
//!
//! and import:
//!
//! ```rust
//! extern crate izihawa_hyper_multipart as multipart;
//! ```
//!

mod boundary;
mod client_;
mod error;

pub mod client {
    pub use crate::error::Error;

    /// This module contains data structures for building a multipart/form
    /// body to send a server.
    ///
    pub mod multipart {
        pub use crate::{
            boundary::BoundaryGenerator,
            client_::{Body, Form, CONTENT_TYPE_APPLICATION_X_DIRECTORY, CONTENT_TYPE_MULTIPART_FORM_DATA},
        };
    }
}
