//! Native bindings for [lxc](https://github.com/lxc/lxc) library.
//!
//! # Examples
//!
//! ```rust
//! use std::ffi::CString;
//!
//! fn main() {
//!     let name = CString::new("playground").unwrap();
//!     let configpath = CString::new("/path/to/config").unwrap();
//!     let c = unsafe {
//!         lxc_sys2::lxc_container_new(
//!             name.as_ptr(),
//!             configpath.as_ptr(),
//!         )
//!     };
//!     let c_defined = unsafe { ((*c).is_defined)(c) };
//!     println!(
//!         "Container {:?} is defined: {}",
//!         name,
//!         c_defined
//!     )
//! }
//! ```
//!
//! # Versioning
//!
//! Version `lxc-sys2:1.0.0` correspondents to `lxc:1.0.0`. In case the bindings
//! had a bug a new version `lxc-sys2:1.0.0-1` will be released which still
//! correspondents to `lxc:1.0.0` including the patch to fix the binding
//! related bug.

#![allow(non_camel_case_types)]

mod attach_options;
mod lxccontainer;

pub use attach_options::*;
pub use lxccontainer::*;
