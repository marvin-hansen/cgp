use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, GenericParam, Generics, Ident, ItemStruct, Type};

/// Generates a struct definition with proper handling of generic parameters and lifetimes.
///
/// This function creates a new struct definition based on the provided identifier and generic parameters.
/// It handles two cases:
/// - For structs without generic parameters: Creates a simple unit struct
/// - For structs with generic parameters: Creates a struct with a `PhantomData` field that properly
///   captures all type parameters and lifetimes
///
/// The function ensures proper type safety by:
/// - Removing bounds from type parameters to avoid unnecessary constraints
/// - Converting lifetime parameters into reference types with appropriate lifetimes
/// - Using `PhantomData` to maintain variance without adding runtime overhead
///
/// # Arguments
///
/// * `ident` - The identifier for the new struct
/// * `generics` - The generic parameters specification, which may include type parameters,
///                lifetime parameters, and const generics
///
/// # Returns
///
/// Returns a [`syn::ItemStruct`] representing the complete struct definition with all
/// necessary generic parameters and phantom data fields.
///
/// # Examples
///
/// ```rust
/// # use syn::{parse_quote, Generics, Ident};
/// # use cgp_component_macro_lib::delegate_components::define_struct::define_struct;
/// // Simple struct without generics
/// let simple_ident: Ident = parse_quote!(SimpleStruct);
/// let empty_generics: Generics = parse_quote!();
/// let simple = define_struct(&simple_ident, &empty_generics);
/// // Results in: pub struct SimpleStruct;
///
/// // Generic struct with type parameter and lifetime
/// let generic_ident: Ident = parse_quote!(GenericStruct);
/// let generics: Generics = parse_quote!(<T, 'a>);
/// let generic = define_struct(&generic_ident, &generics);
/// // Results in: pub struct GenericStruct<T, 'a>(pub ::core::marker::PhantomData<(T, &'a ())>);
/// ```
///
/// # Note
///
/// The generated struct will always be public (`pub`) and will use `PhantomData`
/// to properly handle generic parameters without introducing runtime overhead.

pub fn define_struct(ident: &Ident, generics: &Generics) -> ItemStruct {
    if generics.params.is_empty() {
        parse_quote! {
            pub struct #ident;
        }
    } else {
        let mut generic_params = generics.params.clone();
        let mut phantom_params: Punctuated<Type, Comma> = Default::default();

        for param in generic_params.iter_mut() {
            match param {
                GenericParam::Type(type_param) => {
                    type_param.colon_token = None;
                    type_param.bounds.clear();

                    let type_ident = &type_param.ident;
                    phantom_params.push(parse_quote!( #type_ident ));
                }
                GenericParam::Lifetime(life_param) => {
                    life_param.colon_token = None;
                    life_param.bounds.clear();

                    let lifetime = &life_param.lifetime;
                    phantom_params.push(parse_quote!( & #lifetime () ));
                }
                _ => {}
            }
        }

        parse_quote! {
            pub struct #ident < #generic_params > (
                pub ::core::marker::PhantomData<( #phantom_params )>
            );
        }
    }
}
