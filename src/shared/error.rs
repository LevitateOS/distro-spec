//! Shared error handling framework for rec* tools.
//!
//! Provides a standardized error pattern used by recstrap, recchroot, recfstab,
//! and other installer tools. Each tool defines its own `ErrorCode` enum but
//! uses this shared infrastructure for consistency.
//!
//! # Example
//!
//! ```rust,ignore
//! use distro_spec::shared::error::{ToolError, ToolErrorCode};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! pub enum ErrorCode {
//!     TargetNotFound,
//!     NotADirectory,
//! }
//!
//! impl ToolErrorCode for ErrorCode {
//!     fn code(&self) -> &'static str {
//!         match self {
//!             ErrorCode::TargetNotFound => "E001",
//!             ErrorCode::NotADirectory => "E002",
//!         }
//!     }
//!
//!     fn exit_code(&self) -> u8 {
//!         match self {
//!             ErrorCode::TargetNotFound => 1,
//!             ErrorCode::NotADirectory => 2,
//!         }
//!     }
//! }
//!
//! // Use the generic ToolError
//! type MyError = ToolError<ErrorCode>;
//! ```

use std::fmt;

/// Trait for tool-specific error codes.
///
/// Each rec* tool implements this trait for its own `ErrorCode` enum.
/// This standardizes the interface while allowing tool-specific error variants.
pub trait ToolErrorCode: fmt::Debug + Clone + Copy + PartialEq + Eq {
    /// Get the error code string (e.g., "E001", "E002").
    fn code(&self) -> &'static str;

    /// Get the numeric exit code for this error.
    fn exit_code(&self) -> u8;
}

/// Generic error type for rec* tools.
///
/// Combines an error code with a human-readable message.
/// Implements `Display` as "{code}: {message}" and `std::error::Error`.
#[derive(Debug)]
pub struct ToolError<C: ToolErrorCode> {
    /// The error code identifying this error type.
    pub code: C,
    /// Human-readable error message with context.
    pub message: String,
}

impl<C: ToolErrorCode> ToolError<C> {
    /// Create a new error with the given code and message.
    pub fn new(code: C, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Get the exit code for this error.
    pub fn exit_code(&self) -> u8 {
        self.code.exit_code()
    }
}

impl<C: ToolErrorCode> fmt::Display for ToolError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code.code(), self.message)
    }
}

impl<C: ToolErrorCode> std::error::Error for ToolError<C> {}

/// Helper macro to implement Display for ErrorCode enums.
///
/// Since ErrorCode enums just display their code string, this is a common pattern.
#[macro_export]
macro_rules! impl_error_code_display {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    <Self as $crate::shared::error::ToolErrorCode>::code(self)
                )
            }
        }
    };
}
