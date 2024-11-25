// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Url

use alloc::string::String;
use core::convert::Infallible;
use core::fmt::{self, Debug};
use core::str::FromStr;

pub use url::*;

/// Try into [`Url`]
pub trait TryIntoUrl {
    /// Error
    type Err: Debug;

    /// Try into [`Url`]
    fn try_into_url(self) -> Result<Url, Self::Err>;
}

impl TryIntoUrl for Url {
    type Err = Infallible;

    #[inline]
    fn try_into_url(self) -> Result<Url, Self::Err> {
        Ok(self)
    }
}

impl TryIntoUrl for &Url {
    type Err = Infallible;

    #[inline]
    fn try_into_url(self) -> Result<Url, Self::Err> {
        Ok(self.clone())
    }
}

impl TryIntoUrl for String {
    type Err = ParseError;

    #[inline]
    fn try_into_url(self) -> Result<Url, Self::Err> {
        Url::parse(&self)
    }
}

impl TryIntoUrl for &String {
    type Err = ParseError;

    #[inline]
    fn try_into_url(self) -> Result<Url, Self::Err> {
        Url::parse(self)
    }
}

impl TryIntoUrl for &str {
    type Err = ParseError;

    #[inline]
    fn try_into_url(self) -> Result<Url, Self::Err> {
        Url::parse(self)
    }
}

/// Unchecked Url
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UncheckedUrl(String);

impl UncheckedUrl {
    /// New unchecked url
    #[inline]
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self(url.into())
    }

    /// Empty unchecked url
    #[inline]
    pub fn empty() -> Self {
        Self(String::new())
    }

    /// Get unchecked url as `&str`
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<S> From<S> for UncheckedUrl
where
    S: Into<String>,
{
    #[inline]
    fn from(url: S) -> Self {
        Self(url.into())
    }
}

impl FromStr for UncheckedUrl {
    type Err = ParseError;

    #[inline]
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(url))
    }
}

impl TryFrom<UncheckedUrl> for Url {
    type Error = ParseError;

    #[inline]
    fn try_from(unchecked_url: UncheckedUrl) -> Result<Url, Self::Error> {
        Self::parse(&unchecked_url.0)
    }
}

impl fmt::Display for UncheckedUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_unchecked_relay_url() {
        let relay = "wss://relay.damus.io/";
        let relay_url = Url::from_str(relay).unwrap();

        let unchecked_relay_url = UncheckedUrl::from(relay_url.clone());

        assert_eq!(unchecked_relay_url, UncheckedUrl::from(relay));

        assert_eq!(
            Url::try_from(unchecked_relay_url.clone()).unwrap(),
            relay_url
        );

        assert_eq!(relay, unchecked_relay_url.to_string());
    }
}
