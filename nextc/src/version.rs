//! Version information of NextC.
//!
//! This module provides the version number, release channel, and current
//! platform of NextC.
//!
//! # License
//!
//! Apache-2.0 OR MIT
//!
//! # Authors
//!
//! - Glacyend <Glacyend@outlook.com>

use indoc::indoc;
use std::fmt::{self, Display, Formatter};

pub use current_platform::CURRENT_PLATFORM;

/// The version number of NextC, in `MAJOR.MINOR.PATCH` format.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The release channel of NextCl, e.g. `stable`, `beta`, or `nightly`.
pub const RELEASE_CHANNEL: &str = env!("RELEASE_CHANNEL");

/// The full version string of NextC.
///
/// This constant combines the version number and the release channel in the
/// format `{VERSION}-{RELEASE_CHANNEL}`, for example `0.0.1-nightly` or
/// `0.0.2-beta`.
///
/// **Note**: If the release channel is `stable`, it is omitted, so the full
/// version equals the version number in that case.
pub const FULL_VERSION: &str = env!("FULL_VERSION");

/// Version and platform information of NextC.
///
/// This struct implements the [`Display`] trait. Its [`Display::fmt`]
/// implementation writes the one-line summary from [`Self::version_one_line`]
/// followed by the detailed report from [`Self::version_detail`]:
///
/// ```text
/// NextC 0.0.1-nightly arm64-unknown-linux-gnu
/// full version: 0.0.1-nightly
/// version number: 0.0.1
/// release channel: nightly
/// platform: arm64-unknown-linux-gnu
/// ```
///
/// **Note**: The output ends with a trailing newline.
pub struct VersionInfo;

impl VersionInfo {
    /// Returns a one-line summary of the project name, full version, and
    /// platform.
    pub fn version_one_line() -> String {
        format!("NextC {FULL_VERSION} {CURRENT_PLATFORM}")
    }

    /// Returns detailed version and platform information in multiple lines.
    ///
    /// The returned string has the following format:
    ///
    /// ```text
    /// full version: x.x.x-channel
    /// version number: x.x.x
    /// release channel: channel
    /// platform: xxx-xxx-xxx-xxx
    /// ```
    ///
    /// **Note**: The returned string ends with a newline.
    pub fn version_detail() -> String {
        format!(
            indoc! {"
                full version: {}
                version number: {}
                release channel: {}
                platform: {}
            "},
            FULL_VERSION, VERSION, RELEASE_CHANNEL, CURRENT_PLATFORM,
        )
    }
}

impl Display for VersionInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", Self::version_one_line())?;
        write!(f, "{}", Self::version_detail())?;

        Ok(())
    }
}
