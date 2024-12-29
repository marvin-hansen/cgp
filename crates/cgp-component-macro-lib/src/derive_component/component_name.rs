use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Ident, ItemStruct};

/// Generates a component name struct definition with optional generic parameters.
///
/// This function creates a struct that represents a component's type name in the CGP framework.
/// For components without generic parameters, it generates a simple unit struct.
/// For generic components, it generates a struct with a `PhantomData` field to carry the generic parameters.
///
/// # Arguments
///
/// * `component_name` - The identifier for the component struct
/// * `component_params` - A comma-separated list of generic parameter identifiers
///
/// # Returns
///
/// A `syn::ItemStruct` representing either:
/// - A unit struct `struct ComponentName;` for non-generic components
/// - A generic struct `struct ComponentName<T>(PhantomData<(T)>);` for generic components
///
/// # Examples
///
/// ```rust,ignore
/// // For a non-generic component
/// derive_component_name_struct(&parse_quote!(MyComponent), &Punctuated::new())
/// // Generates: pub struct MyComponent;
///
/// // For a generic component
/// let mut params = Punctuated::new();
/// params.push(parse_quote!(T));
/// derive_component_name_struct(&parse_quote!(MyComponent), &params)
/// // Generates: pub struct MyComponent<T>(pub PhantomData<(T)>);
/// ```
pub fn derive_component_name_struct(
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
) -> ItemStruct {
    if component_params.is_empty() {
        parse_quote!(pub struct #component_name ;)
    } else {
        parse_quote!(pub struct #component_name < #component_params > ( pub core::marker::PhantomData<( #component_params )> );)
    }
}
