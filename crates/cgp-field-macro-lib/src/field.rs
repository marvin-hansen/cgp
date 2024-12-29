/*!
   Field access trait derivation implementation.

   This module implements the derivation of field access traits for structs.
   It generates the necessary trait implementations for both immutable and
   mutable field access using type-safe field identifiers.
*/

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Fields, ItemImpl, ItemStruct};

use crate::symbol::symbol_from_string;

/// Generate field access trait implementations for a struct.
///
/// This function generates implementations of `HasField` and `HasFieldMut`
/// for each named field in the input struct. It creates type-safe field
/// access using symbol types as field identifiers.
///
/// # Arguments
///
/// * `item_struct` - The struct AST for which to generate implementations
///
/// # Returns
///
/// A vector of implementation blocks for the field access traits.
///
/// # Generated Code
///
/// For each field, generates implementations like:
///
/// ```rust,ignore
/// impl HasField<FieldSymbol> for Struct {
///     type Value = FieldType;
///     fn get_field(&self, _: PhantomData<FieldSymbol>) -> &Self::Value {
///         &self.field
///     }
/// }
///
/// impl HasFieldMut<FieldSymbol> for Struct {
///     fn get_field_mut(&mut self, _: PhantomData<FieldSymbol>) -> &mut Self::Value {
///         &mut self.field
///     }
/// }
/// ```
pub fn derive_has_field_impls(item_struct: &ItemStruct) -> Vec<ItemImpl> {
    let struct_ident = &item_struct.ident;

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let mut item_impls = Vec::new();

    if let Fields::Named(fields) = &item_struct.fields {
        for field in fields.named.iter() {
            let field_ident = field.ident.as_ref().unwrap();

            let field_symbol = symbol_from_string(&field_ident.to_string());

            let field_type = &field.ty;

            let has_field_impl: ItemImpl = parse_quote! {
                impl #impl_generics HasField< #field_symbol >
                    for #struct_ident #ty_generics
                #where_clause
                {
                    type Value = #field_type;

                    fn get_field(
                        &self,
                        key: ::core::marker::PhantomData< #field_symbol >,
                    ) -> &Self::Value
                    {
                        &self. #field_ident
                    }
                }
            };

            let has_field_mut_impl: ItemImpl = parse_quote! {
                impl #impl_generics HasFieldMut< #field_symbol >
                    for #struct_ident #ty_generics
                #where_clause
                {
                    fn get_field_mut(
                        &mut self,
                        key: ::core::marker::PhantomData< #field_symbol >,
                    ) -> &mut Self::Value
                    {
                        &mut self. #field_ident
                    }
                }
            };

            item_impls.push(has_field_impl);
            item_impls.push(has_field_mut_impl);
        }
    }

    item_impls
}

/// Derive field access traits for a struct.
///
/// This function is the main entry point for the field access trait
/// derivation. It parses the input token stream as a struct definition
/// and generates the necessary trait implementations.
///
/// # Arguments
///
/// * `input` - Token stream containing the struct definition
///
/// # Returns
///
/// Token stream containing the generated trait implementations
///
/// # Generated Traits
///
/// * `HasField` - For immutable field access
/// * `HasFieldMut` - For mutable field access
pub fn derive_fields(input: TokenStream) -> TokenStream {
    let item_struct: ItemStruct = syn::parse2(input).unwrap();

    let item_impls = derive_has_field_impls(&item_struct);

    let mut output = TokenStream::new();

    for item_impl in item_impls {
        output.extend(item_impl.to_token_stream());
    }

    output
}
