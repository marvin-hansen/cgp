/// Core functionality for deriving component traits and implementations.
///
/// This module handles the generation of all necessary types and implementations
/// for component-based programming patterns.
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemTrait;

use crate::derive_component::component_name::derive_component_name_struct;
use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::consumer_impl::derive_consumer_impl;
use crate::derive_component::provider_impl::derive_provider_impl;
use crate::derive_component::provider_trait::derive_provider_trait;

/// Derives a complete component implementation from a trait definition.
///
/// This function is the main entry point for the component derivation process.
/// It generates all the necessary types and implementations for a component:
/// - A component name struct
/// - A provider trait
/// - Consumer implementations
/// - Provider implementations
///
/// # Arguments
/// * `attr` - Attribute tokens containing component specification
/// * `item` - The input trait definition tokens
///
/// # Returns
/// * `TokenStream` - Generated code containing all component implementations
///
/// # Generated Items
/// For a component named 'MyComponent':
/// ```ignore
/// // Component name struct
/// pub struct MyComponent;
///
/// // Provider trait
/// pub trait MyComponentProvider<Context> { ... }
///
/// // Consumer implementation
/// impl<T: MyComponentProvider<Context>> MyComponent for T { ... }
///
/// // Provider implementation
/// impl<T> MyComponentProvider<Context> for T where T: Provider<MyComponent> { ... }
/// ```
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let spec: ComponentSpec = syn::parse2(attr).unwrap();

    let consumer_trait: ItemTrait = syn::parse2(item).unwrap();

    let provider_name = &spec.provider_name;
    let context_type = &spec.context_type;

    let component_struct =
        derive_component_name_struct(&spec.component_name, &spec.component_params);

    let provider_trait =
        derive_provider_trait(&consumer_trait, provider_name, context_type).unwrap();

    let consumer_impl = derive_consumer_impl(&consumer_trait, provider_name, context_type);

    let provider_impl = derive_provider_impl(
        &provider_trait,
        &spec.component_name,
        &spec.component_params,
    );

    let mut output = consumer_trait.to_token_stream();

    output.extend(component_struct.to_token_stream());
    output.extend(provider_trait.to_token_stream());
    output.extend(consumer_impl.to_token_stream());
    output.extend(provider_impl.to_token_stream());

    output
}
