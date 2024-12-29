/// Abstract Syntax Tree definitions for preset parsing.
///
/// This module provides the AST structures necessary for parsing preset definitions
/// in the CGP framework.
use syn::parse::{Parse, ParseStream};
use syn::token::Lt;
use syn::{Generics, Ident};

use crate::delegate_components::ast::DelegateEntriesAst;

/// AST node representing a complete preset definition.
///
/// Captures all the necessary information for defining a preset, including its
/// identifier, generic parameters, and delegation specifications.
///
/// # Fields
///
/// * `preset_ident` - The identifier for the preset being defined
/// * `preset_generics` - Generic parameters associated with the preset
/// * `delegate_entries` - Specifications for component delegations within the preset
///
/// # Examples
///
/// ```text
/// MyPreset<T> {
///     [ComponentA, ComponentB]: Inner<T>
/// }
/// ```
pub struct DefinePresetAst {
    pub preset_ident: Ident,
    pub preset_generics: Generics,
    pub delegate_entries: DelegateEntriesAst,
}

/// Parse implementation for preset definitions.
///
/// Parses input in the format:
/// ```text
/// preset_name<generic_params>? { delegate_entries }
/// ```
///
/// The generic parameters are optional, and delegate entries follow the same
/// format as in component delegation.
impl Parse for DefinePresetAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset_ident: Ident = input.parse()?;

        let preset_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let delegate_entries: DelegateEntriesAst = input.parse()?;

        Ok(Self {
            preset_ident,
            preset_generics,
            delegate_entries,
        })
    }
}
