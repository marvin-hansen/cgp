/*!
   Procedural macros for field manipulation in the CGP framework.

   This crate provides macros for working with fields in algebraic data types,
   enabling type-safe field access, product and sum types, and symbol handling.

   # Macros

   ## Field Access

   * [`macro@HasField`] - Derive macro for implementing field access traits.
     Enables type-safe field access and manipulation.

   ## Type Construction

   * [`macro@Product`] - Macro for defining product types (structs).
   * [`macro@Sum`] - Macro for defining sum types (enums).
   * [`macro@product`] - Macro for constructing product type expressions.

   ## Symbol Handling

   * [`macro@symbol`] - Macro for creating type-safe symbol references.

   # Examples

   ## Field Access

   ```rust,ignore
   use cgp_field_macro::HasField;

   #[derive(HasField)]
   struct Person {
       name: String,
       age: u32,
   }
   ```

   ## Product Types

   ```rust,ignore
   use cgp_field_macro::Product;

   Product! {
       struct Point {
           x: f64,
           y: f64,
       }
   }
   ```

   ## Sum Types

   ```rust,ignore
   use cgp_field_macro::Sum;

   Sum! {
       enum Shape {
           Circle(f64),
           Rectangle(f64, f64),
       }
   }
   ```

   ## Symbol Creation

   ```rust,ignore
   use cgp_field_macro::symbol;

   let name_symbol = symbol!(name);
   ```
*/

extern crate proc_macro;

use proc_macro::TokenStream;

/// Derive macro for implementing field access traits.
///
/// This macro generates implementations for field access traits, enabling
/// type-safe access to struct fields. It analyzes the struct's fields and
/// generates the necessary trait implementations for each field.
///
/// # Examples
///
/// ```rust,ignore
/// #[derive(HasField)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// // Generated code enables field access like:
/// let person = Person { name: "Alice".into(), age: 30 };
/// let name = person.get_field(field!(name));
/// ```
#[proc_macro_derive(HasField)]
pub fn derive_fields(item: TokenStream) -> TokenStream {
    cgp_field_macro_lib::derive_fields(item.into()).into()
}

/// Macro for creating type-safe symbol references.
///
/// This macro creates symbol types that can be used for type-safe field
/// access and manipulation. Symbols are typically used in conjunction with
/// field access traits.
///
/// # Examples
///
/// ```rust,ignore
/// let name_symbol = symbol!(name);
/// let age_symbol = symbol!(age);
///
/// // Use symbols for field access
/// let person_name = person.get_field(name_symbol);
/// ```
#[proc_macro]
pub fn symbol(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_symbol(body.into()).into()
}

/// Macro for defining product types (structs).
///
/// This macro provides a convenient way to define product types with
/// automatically generated field access implementations. It supports
/// all standard struct features while adding CGP field capabilities.
///
/// # Examples
///
/// ```rust,ignore
/// Product! {
///     struct Point {
///         x: f64,
///         y: f64,
///     }
/// }
///
/// // Generated type can be used with field access:
/// let point = Point { x: 1.0, y: 2.0 };
/// let x = point.get_field(field!(x));
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn Product(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_product_type(body.into()).into()
}

/// Macro for defining sum types (enums).
///
/// This macro provides a convenient way to define sum types with
/// automatically generated variant handling. It supports all standard
/// enum features while adding CGP field capabilities.
///
/// # Examples
///
/// ```rust,ignore
/// Sum! {
///     enum Shape {
///         Circle(f64),
///         Rectangle(f64, f64),
///     }
/// }
///
/// // Generated type can be used with pattern matching:
/// let shape = Shape::Circle(5.0);
/// match shape {
///     Shape::Circle(r) => println!("Circle with radius {}", r),
///     Shape::Rectangle(w, h) => println!("Rectangle {}x{}", w, h),
/// }
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn Sum(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_sum_type(body.into()).into()
}

/// Macro for constructing product type expressions.
///
/// This macro provides a convenient syntax for constructing instances of
/// product types. It works in conjunction with the Product macro to provide
/// a consistent interface for working with product types.
///
/// # Examples
///
/// ```rust,ignore
/// let point = product! {
///     Point {
///         x: 1.0,
///         y: 2.0,
///     }
/// };
/// ```
#[proc_macro]
pub fn product(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_product_expr(body.into()).into()
}
