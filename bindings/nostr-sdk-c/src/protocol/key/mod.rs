// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ffi::{c_char, CStr, CString};

use nostr_sdk::nostr::key;

use crate::error::{self, Result};

pub struct Keys {
    inner: key::Keys,
}

impl Keys {
    #[no_mangle]
    pub extern "C" fn keys_generate() -> *mut Keys {
        Box::into_raw(Box::new(Keys {
            inner: key::Keys::generate(),
        }))
    }

    #[no_mangle]
    pub unsafe extern "C" fn keys_parse(secret_key: *const c_char) -> Result<Keys> {
        error::handle_result(|| {
            let secret_key: &str = CStr::from_ptr(secret_key).to_str()?;
            Ok(Keys {
                inner: key::Keys::parse(secret_key)?,
            })
        })
    }

    /// Get public key from Keys
    #[no_mangle]
    pub extern "C" fn keys_public_key(&self) -> *const c_char {
        let public_key = self.inner.public_key.to_string();
        let c_string = CString::new(public_key).unwrap();
        c_string.into_raw()
    }
}
