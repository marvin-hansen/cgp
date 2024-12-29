/// Abstract Syntax Tree (AST) structures for component delegation.
///
/// This module provides the AST representation for parsing and processing
/// component delegation syntax in the CGP framework.

use core::iter;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Lt};
use syn::{braced, bracketed, Generics, Token, Type};

/// Root AST node representing a complete component delegation specification.
///
/// This structure captures the target type that will implement the delegated
/// components, its generic parameters, and the entries specifying which
/// components are delegated to which sources.
///
/// # Fields
///
/// * `target_type` - The type that will implement the delegated components
/// * `target_generics` - Generic parameters for the target type
/// * `delegate_entries` - Collection of delegation specifications
pub struct DelegateComponentsAst {
    pub target_type: Type,
    pub target_generics: Generics,
    pub delegate_entries: DelegateEntriesAst,
}

/// Collection of delegate entries in the AST.
///
/// Represents a comma-separated list of delegate entries, where each entry
/// specifies which components are delegated to a particular source.
///
/// # Fields
///
/// * `entries` - Punctuated sequence of delegate entries
pub struct DelegateEntriesAst {
    pub entries: Punctuated<DelegateEntryAst, Comma>,
}

/// Single delegation entry in the AST.
///
/// Specifies a mapping between a set of components and their source type
/// for delegation.
///
/// # Fields
///
/// * `components` - List of components to be delegated
/// * `source` - The type that provides the component implementations
pub struct DelegateEntryAst {
    pub components: Punctuated<ComponentAst, Comma>,
    pub source: Type,
}

/// AST node representing a single component specification.
///
/// Captures a component type and its associated generic parameters.
///
/// # Fields
///
/// * `component_type` - The type of the component being delegated
/// * `component_generics` - Generic parameters for the component
#[derive(Clone)]
pub struct ComponentAst {
    pub component_type: Type,
    pub component_generics: Generics,
}

impl DelegateEntriesAst {
    /// Collects all components from all delegate entries into a single sequence.
    ///
    /// # Returns
    ///
    /// Returns a `Punctuated` sequence containing all components across all entries,
    /// preserving their order of appearance.
    pub fn all_components(&self) -> Punctuated<ComponentAst, Comma> {
        self.entries
            .iter()
            .flat_map(|entry| entry.components.clone().into_iter())
            .collect()
    }
}

/// Parse implementation for delegate components AST.
///
/// Parses input in the format:
/// ```text
/// <generics>? target_type { entries... }
/// ```
impl Parse for DelegateComponentsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let target_type: Type = input.parse()?;

        let delegate_entries: DelegateEntriesAst = input.parse()?;

        Ok(Self {
            target_type,
            target_generics,
            delegate_entries,
        })
    }
}

/// Parse implementation for delegate entries.
///
/// Parses input in the format:
/// ```text
/// { entry1, entry2, ... }
/// ```
impl Parse for DelegateEntriesAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entries = {
            let entries_body;
            braced!(entries_body in input);
            entries_body.parse_terminated(DelegateEntryAst::parse, Comma)?
        };

        Ok(Self { entries })
    }
}

/// Parse implementation for a single delegate entry.
///
/// Parses input in the format:
/// ```text
/// [comp1, comp2, ...]: source_type
/// ```
/// or
/// ```text
/// comp: source_type
/// ```
impl Parse for DelegateEntryAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(ComponentAst::parse, Token![,])?
        } else {
            let component: ComponentAst = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let _: Colon = input.parse()?;

        let source: Type = input.parse()?;

        Ok(Self { components, source })
    }
}

/// Parse implementation for component specification.
///
/// Parses input in the format:
/// ```text
/// <generics>? component_type
/// ```
impl Parse for ComponentAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let component_type: Type = input.parse()?;

        Ok(Self {
            component_type,
            component_generics,
        })
    }
}

/// Token stream generation for component specifications.
///
/// Converts a component specification into a token stream by concatenating
/// its generic parameters and component type.
impl ToTokens for ComponentAst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.component_generics.to_token_stream());
        tokens.extend(self.component_type.to_token_stream());
    }
}
