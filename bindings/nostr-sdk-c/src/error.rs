// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ffi::{c_char, CString};

use nostr_sdk::Result as UniversalResult;

#[repr(C)]
pub struct Result<T> {
    pub success: bool,
    pub output: ResultOutput<T>,
}

impl<T> Result<T> {
    fn ok(val: T) -> Self {
        Self {
            success: true,
            output: ResultOutput {
                ok: Box::into_raw(Box::new(val)),
            },
        }
    }

    fn err(e: String) -> Self {
        let c_string = CString::new(e).unwrap();
        Self {
            success: false,
            output: ResultOutput {
                err: c_string.into_raw(),
            },
        }
    }
}

#[repr(C)]
pub union ResultOutput<T> {
    ok: *const T,
    err: *const c_char,
}

#[inline]
pub fn handle_result<F, T>(f: F) -> Result<T>
where
    F: FnOnce() -> UniversalResult<T>,
{
    match f() {
        Ok(value) => Result::ok(value),
        Err(e) => Result::err(e.to_string()),
    }
}
