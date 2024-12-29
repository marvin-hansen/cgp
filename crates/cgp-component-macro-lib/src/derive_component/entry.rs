/// Parsing utilities for key-value entries in component specifications.
///
/// This module provides structures and parsing implementations for handling
/// key-value pairs in component macro syntax, such as field specifications
/// and component configurations.
use std::collections::BTreeMap;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma};
use syn::{Ident, Type};

/// A single key-value entry in a component specification.
///
/// # Fields
///
/// * `key` - The identifier key of the entry
/// * `value` - The type associated with the key
///
/// # Example
///
/// ```rust,ignore
/// field_name: FieldType
/// ```
pub struct Entry {
    pub key: Ident,
    pub value: Type,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let _colon: Colon = input.parse()?;
        let value = input.parse()?;

        Ok(Entry { key, value })
    }
}

/// A collection of key-value entries stored in a sorted map.
///
/// # Fields
///
/// * `entries` - A BTreeMap storing entries sorted by their key identifiers
///
/// # Example
///
/// ```rust,ignore
/// field1: Type1,
/// field2: Type2,
/// field3: Type3
/// ```
///
/// # Note
///
/// Using BTreeMap ensures consistent ordering of entries, which is important
/// for deterministic code generation.
pub struct Entries {
    pub entries: BTreeMap<Ident, Type>,
}

impl Parse for Entries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entry_list: Punctuated<Entry, Comma> = Punctuated::parse_terminated(input)?;

        let entries =
            BTreeMap::from_iter(entry_list.into_iter().map(|entry| (entry.key, entry.value)));

        Ok(Entries { entries })
    }
}
