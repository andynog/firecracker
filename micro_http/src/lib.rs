// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![deny(missing_docs)]
//! Minimal implementation of the [HTTP/1.0](https://tools.ietf.org/html/rfc1945)
//! and [HTTP/1.1](https://www.ietf.org/rfc/rfc2616.txt) protocols.
//!
//! HTTP/1.1 has a mandatory header **Host**, but as this crate is only used
//! for parsing MMDS requests, this header (if present) is ignored.
//!
//! This HTTP implementation is stateless thus it does not support chunking or
//! compression.
//!
//! ## Supported Headers
//! The **micro_http** crate does not have support for parsing **Request**
//! headers.
//!
//! The **Response** does not have a public interface for adding headers, but whenever
//! a write to the **Body** is made, the headers **ContentLength** and **MediaType**
//! are automatically updated.
//!
//! ### Media Types
//! The only supported media type is **text/plain**.
//!
//! ## Supported Methods
//! The only supported HTTP Method is **GET**.
//!
//! ## Supported Status Codes
//! The supported status codes are:
//!
//! - OK - 200
//! - Bad Request - 400
//! - Not Found - 404
//! - Internal Server Error - 500
//! - Not Implemented - 501
//!
//! ## Example for parsing an HTTP Request from a slice
//! ```
//! extern crate micro_http;
//! use micro_http::{Request, Version};
//!
//! let http_request = Request::try_from(b"GET http://localhost/home HTTP/1.0\r\n").unwrap();
//! assert_eq!(http_request.http_version(), Version::Http10);
//! assert_eq!(http_request.uri().get_abs_path(), "/home");
//! ```
//!
//! ## Example for creating an HTTP Response
//! ```
//! extern crate micro_http;
//! use micro_http::{Body, Response, StatusCode, Version};
//!
//! let mut response = Response::new(Version::Http10, StatusCode::OK);
//! let body = String::from("This is a test");
//! response.set_body(Body::new(body.clone()));
//!
//! assert!(response.status() == StatusCode::OK);
//! assert_eq!(response.body().unwrap(), Body::new(body));
//! assert_eq!(response.http_version(), Version::Http10);
//!
//! let mut response_buf: [u8; 77] = [0; 77];
//! assert!(response.write_all(&mut response_buf.as_mut()).is_ok());
//! ```
mod common;
mod request;
mod response;
use common::ascii;
use common::headers;

pub use request::{Request, RequestError};
pub use response::{Response, StatusCode};

pub use common::{Body, Version};
