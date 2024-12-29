/// Implementation generation for preset trait bounds.
///
/// This module provides functionality for generating trait implementations that
/// establish the relationship between presets and their component types.
use syn::{parse_quote, Generics, Ident, ItemImpl, Type};

use crate::delegate_components::ast::{ComponentAst, DelegateEntriesAst};

/// Generates implementations of the preset trait for all components.
///
/// Creates trait implementations that establish which components are part
/// of a preset, allowing the preset to be used with those component types.
///
/// # Arguments
///
/// * `trait_name` - Name of the preset trait (e.g., `IsMyPreset`)
/// * `preset_type` - Type of the preset
/// * `preset_generics` - Generic parameters for the preset
/// * `delegate_entries` - Component delegation specifications
///
/// # Returns
///
/// Returns a vector of trait implementations, one for each component in
/// the preset's delegation specifications.
pub fn impl_components_is_preset(
    trait_name: &Ident,
    preset_type: &Type,
    preset_generics: &Generics,
    delegate_entries: &DelegateEntriesAst,
) -> Vec<ItemImpl> {
    delegate_entries
        .entries
        .iter()
        .flat_map(|entry| {
            entry.components.iter().map(|component| {
                impl_component_is_preset(trait_name, preset_type, preset_generics, component)
            })
        })
        .collect()
}

/// Generates a single preset trait implementation for a component.
///
/// Creates the implementation that establishes that a specific component
/// type can be used with the preset.
///
/// # Arguments
///
/// * `trait_name` - Name of the preset trait
/// * `_preset_type` - Type of the preset (currently unused)
/// * `_preset_generics` - Generic parameters for the preset (currently unused)
/// * `component` - The component to implement the preset trait for
///
/// # Returns
///
/// Returns an implementation of the preset trait for the specified component.
///
/// # Note
///
/// The preset generic parameter may be absent if it is used as part of the
/// component name's generic parameters. This is a known limitation that
/// needs to be addressed in future updates.
pub fn impl_component_is_preset(
    trait_name: &Ident,
    _preset_type: &Type,
    _preset_generics: &Generics,
    component: &ComponentAst,
) -> ItemImpl {
    let component_type = &component.component_type;

    // FIXME: The preset generic would be absent if the if it is used as part of the
    // component name's generic.
    // let generics = merge_generics(preset_generics, &component.component_generics);

    let mut generics = component.component_generics.clone();
    generics.params.push(parse_quote!(T));

    let impl_generics = generics.split_for_impl().0;

    parse_quote! {
        impl #impl_generics #trait_name < #component_type > for T {}
    }
}
