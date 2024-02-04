#![no_std]

use alloc::{format, string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    Key, URef,
};
use token::TokenIdentifier;

extern crate alloc;

pub mod prelude {
    pub use alloc::{string::String, vec, vec::Vec};

    pub use casper_types::{
        api_error,
        bytesrepr::{self, FromBytes, ToBytes},
        ApiError, CLValue, ContractHash, ContractPackageHash, Key, URef, U128, U256, U512,
    };

    pub use casper_contract::{
        contract_api::{
            self,
            runtime::{self, get_caller, get_key, get_named_arg, ret, revert},
            storage,
        },
        ext_ffi,
        unwrap_or_revert::UnwrapOrRevert,
    };

    pub use crate::{
        contract_api::try_get_named_arg, ensure, ensure_eq, ensure_neq, entrypoint,
        entrypoints, error::CommonError, forward_entrypoints, named_arg, named_key, named_keys,
        serializable_structs, st_non_sync_static,
    };
}

pub mod call_stack;
pub mod contract_api;
pub mod error;
pub mod ext;
pub mod macros;
pub mod token;

pub trait ToStrKey {
    fn to_key(&self) -> String;
}

impl<T: AsRef<[u8]>> ToStrKey for T {
    fn to_key(&self) -> String {
        String::from_utf8_lossy(self.as_ref()).into_owned()
    }
}

// TODO: Remove this function altogether from the project
// pub fn b64<T: AsRef<[u8]>>(input: T) -> alloc::string::String {
//     // TODO: Remove these comments
//     // use base64::Engine;
//     // base64::prelude::BASE64_STANDARD_NO_PAD.encode(input.as_ref())
//     input.to_key()
// }

// pub fn b64_cl<T: ToBytes>(input: &T) -> alloc::string::String {
//     b64(input.to_bytes().unwrap())
// }

pub fn store_named_key_incremented(key: Key, name: &str) {
    for i in 0.. {
        let formatted_name = match i {
            0 => format!("{name}"),
            _ => format!("{name}_{i}")
        };

        if runtime::get_key(&formatted_name).is_none() {
            runtime::put_key(&formatted_name, key);
            break;
        }
    }
}

pub trait FromNamedArg {
    fn try_get(name: &str) -> Option<Self>
    where
        Self: Sized,
        Self: FromBytes,
    {
        crate::contract_api::try_get_named_arg(name).ok()
    }
}

macro_rules! impl_from_named_arg {
    ($($type:ty),*) => {
        $(
            impl FromNamedArg for $type {}
        )*
    };
}

impl_from_named_arg!(
    alloc::string::String,
    bool,
    u8,
    u32,
    u64,
    i32,
    i64,
    URef,
    Key,
    casper_types::U256,
    casper_types::U512,
    casper_types::U128,
    casper_types::ContractHash,
    casper_types::ContractPackageHash
);

impl<T: FromBytes> FromNamedArg for Vec<T> {
    fn try_get(name: &str) -> Option<Self> {
        crate::contract_api::try_get_named_arg(name).ok()
    }
}

impl<T: FromNamedArg + FromBytes> FromNamedArg for Option<T> {
    fn try_get(name: &str) -> Option<Self> {
        Some(T::try_get(name))
    }
}
