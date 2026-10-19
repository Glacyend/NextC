//! build.rs
//!
//! Configure some basic information.
//!
//! # License
//!
//! Apache-2.0 OR MIT
//!
//! # Authors
//!
//! - Glacyend <Glacyend@outlook.com>

use std::env;

fn main() {
    build_data::set_GIT_BRANCH().unwrap();
    build_data::set_GIT_DIRTY().unwrap();
    build_data::rerun_if_git_commit_or_branch_changed().unwrap();

    let branch = build_data::get_git_branch().unwrap_or_default();

    let channel = if branch.starts_with("beta") {
        "beta"
    } else if branch.starts_with("v") {
        "stable"
    } else {
        "nightly"
    };

    let version = env::var("CARGO_PKG_VERSION").unwrap_or_default();
    // Omits release channel if it's stable.
    let full_version = if channel == "stable" {
        version
    } else {
        format!("{version}-{channel}")
    };

    println!("cargo:rustc-env=RELEASE_CHANNEL={channel}");
    println!("cargo:rustc-env=FULL_VERSION={full_version}");
}
