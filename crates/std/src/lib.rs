#![no_std]

extern crate alloc;

pub use alloc::{borrow, boxed, rc, string, sync, vec};

pub mod collections {
    pub use alloc::collections::*;
    pub use hash_map::HashMap;
    pub use hashbrown::hash_map;
    pub use hashbrown::hash_set::HashSet;
}

// ..stub.. don't have this without os support
pub mod path {
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct PathBuf {
        inner: alloc::string::String,
    }
}

// ..stub.. don't have this without os support
pub mod time {
    pub use core::time::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct SystemTime {}
}

pub use core::{
    any, cell, cmp, convert, f32, f64, fmt, hash, iter, mem, ops, option, result, slice, str,
};

pub mod prelude {
    pub use core::prelude as v1;
}

pub use alloc::borrow::ToOwned;
pub use alloc::string::{String, ToString};
pub use alloc::{format, vec::Vec};
