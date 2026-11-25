//! The NextC compiler.
//!
//! NextC is a system programming language inspired by C, offering modern
//! syntax, stronger type safety, zero-cost abstractions, and full low-level
//! control.
//!
//! # License
//!
//! Apache-2.0 OR MIT
//!
//! # Authors
//!
//! - Glacyend <Glacyend@outlook.com>

use nextc::version::VersionInfo;

fn main() {
    print!("{}", VersionInfo);
}
