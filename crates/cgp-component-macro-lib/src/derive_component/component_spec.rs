/// Specification parsing for component attributes and names.
use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};
use syn::{Error, Ident};

use crate::derive_component::entry::Entries;

/// Specification for a component, containing all necessary information for generation.
///
/// This structure holds the parsed information from a component attribute macro,
/// including provider details, context type, and component naming information.
#[derive(Debug)]
pub struct ComponentSpec {
    /// Name of the provider trait that will be generated
    pub provider_name: Ident,
    /// Type of context used in the component
    pub context_type: Ident,
    /// Name of the component struct that will be generated
    pub component_name: Ident,
    /// Generic parameters for the component
    pub component_params: Punctuated<Ident, Comma>,
}

/// Specification for a component's name and its generic parameters.
///
/// This structure represents the parsed form of a component name declaration,
/// which may include generic parameters.
#[derive(Debug)]
pub struct ComponentNameSpec {
    /// Name of the component
    pub component_name: Ident,
    /// Generic parameters for the component
    pub component_params: Punctuated<Ident, Comma>,
}

/// Parser implementation for ComponentSpec
///
/// Parses component specifications in the format:
/// ```ignore
/// #[component(
///     provider = MyProvider,
///     context = MyContext,
///     name = MyComponent<T, U>
/// )]
/// ```
impl Parse for ComponentSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let Entries { entries } = input.parse()?;

        let context_type: Ident = {
            let raw_context_type = entries.get(&Ident::new("context", Span::call_site()));

            if let Some(context_type) = raw_context_type {
                syn::parse2(context_type.to_token_stream())?
            } else {
                Ident::new("Context", Span::call_site())
            }
        };

        let provider_name: Ident = {
            let raw_provider_name = entries
                .get(&Ident::new("provider", Span::call_site()))
                .ok_or_else(|| Error::new(input.span(), "expect provider name to be given"))?;

            syn::parse2(raw_provider_name.to_token_stream())?
        };

        let (component_name, component_params) = {
            let raw_component_name = entries.get(&Ident::new("name", Span::call_site()));

            if let Some(raw_component_name) = raw_component_name {
                let ComponentNameSpec {
                    component_name,
                    component_params,
                } = syn::parse2(raw_component_name.to_token_stream())?;
                (component_name, component_params)
            } else {
                (
                    Ident::new(&format!("{}Component", provider_name), provider_name.span()),
                    Punctuated::default(),
                )
            }
        };

        Ok(ComponentSpec {
            component_name,
            provider_name,
            context_type,
            component_params,
        })
    }
}

/// Parser implementation for ComponentNameSpec
///
/// Parses component names in the format:
/// ```ignore
/// MyComponent<T, U>
/// ```
/// or just:
/// ```ignore
/// MyComponent
/// ```
impl Parse for ComponentNameSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_name: Ident = input.parse()?;

        let component_params = if input.peek(Lt) {
            let _: Lt = input.parse()?;

            let component_params: Punctuated<Ident, Comma> =
                Punctuated::parse_separated_nonempty(input)?;

            let _: Gt = input.parse()?;

            component_params
        } else {
            Punctuated::default()
        };

        Ok(Self {
            component_name,
            component_params,
        })
    }
}
