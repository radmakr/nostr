// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

//! Policies

use std::fmt;

use nostr::util::BoxedFuture;
use nostr::Event;

/// Policy Error
#[derive(Debug)]
pub enum PolicyError {
    /// An error happened in the underlying backend.
    Backend(Box<dyn std::error::Error + Send + Sync>),
}

impl std::error::Error for PolicyError {}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(e) => write!(f, "{e}"),
        }
    }
}

impl PolicyError {
    /// Create a new backend error
    ///
    /// Shorthand for `Error::Backend(Box::new(error))`.
    #[inline]
    pub fn backend<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Backend(Box::new(error))
    }
}

/// Admission status
pub enum AdmitStatus {
    /// Admission succeeds
    Success,
    /// Admission rejected
    Rejected,
}

impl AdmitStatus {
    /// Admission succeeds
    #[inline]
    pub fn success() -> Self {
        Self::Success
    }

    /// Admission rejected
    #[inline]
    pub fn rejected() -> Self {
        Self::Rejected
    }
}

/// Admission policy
pub trait AdmitPolicy: fmt::Debug + Send + Sync {
    /// Admit [`Event`]
    ///
    /// Returns [`AdmitStatus::Success`] if the event is admitted, otherwise [`AdmitStatus::Rejected`].
    fn admit_event<'a>(
        &'a self,
        event: &'a Event,
    ) -> BoxedFuture<'a, Result<AdmitStatus, PolicyError>>;
}
