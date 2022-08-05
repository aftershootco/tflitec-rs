// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod error;
pub mod interpreter;
pub mod model;
pub mod tensor;

pub use self::error::{Error, ErrorKind, Result};

