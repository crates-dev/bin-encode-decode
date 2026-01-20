//! bin-encode-decode
//!
//! A high-performance binary encode and decode library
//! that supports customizable character sets beyond Base64.

pub(crate) mod common;
pub(crate) mod decode;
pub(crate) mod encode;

pub use {common::*, decode::*, encode::*};

pub(crate) use common::r#const::*;

pub(crate) use std::{collections::HashSet, fmt};
