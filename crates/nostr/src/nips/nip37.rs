// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

//! NIP37: Draft Events
//!
//! <https://github.com/nostr-protocol/nips/blob/master/37.md>

use core::fmt;

use crate::{Event, EventBuilder, JsonUtil, Kind, NostrSigner, PublicKey, SignerError, Tag, TagStandard};
use crate::event::builder;

/// Error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Signer error
    Signer(SignerError),
    /// Event builder error
    EventBuilder(builder::Error)
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signer(e) => write!(f, "{e}"),
            Self::EventBuilder(e) => write!(f, "{e}"),
        }
    }
}

impl From<SignerError> for Error {
    fn from(e: SignerError) -> Self {
        Self::Signer(e)
    }
}

impl From<builder::Error> for Error {
    fn from(e: builder::Error) -> Self {
        Self::EventBuilder(e)
    }
}

/// Event draft
pub struct EventDraft {
    /// Identifier of the draft (`d` tag)
    pub id: String,
    /// Event kind
    pub kind: Kind,
    /// Draft content
    pub content: String,
    /// Draft tags
    pub tags: Vec<Tag>,
    /// Additional tags for [`Kind::EventDraft`] event 
    pub additional_public_tags: Vec<Tag>,
}

impl EventDraft {
    /// This method builds the [`Event`] draft to publish to relays
    pub async fn save<T>(&self, signer: &T) -> Result<Event, Error> 
    where
        T: NostrSigner,
    {
        // Get signer public key
        let public_key: PublicKey = signer.get_public_key().await?;
        
        // Build the event draft
        let draft: Event = EventBuilder::new(self.kind, &self.content).tags(self.tags.clone()).sign(signer).await?;
        
        // Encrypt the event draft
        let content: String = signer.nip44_encrypt(&public_key, &draft.as_json()).await?;
        
        // Build the public event
        let mut tags = Vec::with_capacity(2 + self.additional_public_tags.len());
        
        tags.push(Tag::identifier(&self.id));
        tags.push(Tag::from_standardized_without_cell(TagStandard::Kind {kind: self.kind, uppercase: false }));
        tags.extend(self.additional_public_tags.iter().cloned());
        
        let builder: EventBuilder = EventBuilder::new(Kind::EventDraft, content).tags(tags);
        
        Ok(builder.sign(signer).await?)
    }
}