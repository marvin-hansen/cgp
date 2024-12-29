/*!
   Procedural macros for Component-based Generic Programming (CGP).

   This crate provides a set of procedural macros that enable the implementation
   of component-based systems using the CGP framework. These macros automate
   common patterns and reduce boilerplate when working with components.

   # Macros

   ## Component Definition

   * [`macro@cgp_component`] - Attribute macro for defining CGP components. This macro
     processes trait and impl blocks to enable component functionality.

   ## Component Delegation

   * [`macro@delegate_components`] - Macro for implementing component delegation. This
     allows a type to delegate component implementations to its fields.

   ## Component Presets

   * [`macro@cgp_preset`] - Macro for defining reusable component configurations. This
     enables creating preset combinations of components.

   ## Type Replacement

   * [`macro@for_each_replace`] - Utility macro for applying type replacements across
     multiple items.
   * [`macro@replace_with`] - Utility macro for replacing types with other types.

   # Examples

   ## Defining a Component

   ```rust,ignore
   use cgp_component_macro::cgp_component;

   #[cgp_component]
   trait Logger {
       fn log(&self, message: &str);
   }

   #[cgp_component]
   impl Logger for ConsoleLogger {
       fn log(&self, message: &str) {
           println!("{}", message);
       }
   }
   ```

   ## Delegating Components

   ```rust,ignore
   use cgp_component_macro::delegate_components;

   delegate_components! {
       struct MyApp {
           logger: ConsoleLogger,
       }
   }
   ```

   ## Using Presets

   ```rust,ignore
   use cgp_component_macro::cgp_preset;

   cgp_preset! {
       struct LoggingPreset {
           logger: ConsoleLogger,
           metrics: PrometheusMetrics,
       }
   }
   ```

   ## Type Replacement

   ```rust,ignore
   use cgp_component_macro::{for_each_replace, replace_with};

   for_each_replace! {
       [T] in [String, i32, bool] {
           fn process(value: T) -> T { value }
       }
   }

   replace_with! {
       type NewType = OldType<String>;
   }
   */
//*/

extern crate proc_macro;

use proc_macro::TokenStream;

/// Attribute macro for defining CGP components.
///
/// This macro can be applied to both trait definitions and trait implementations
/// to enable CGP component functionality. It processes the input to add necessary
/// trait bounds, associated types, and implementations.
///
/// # Examples
///
/// ```rust,ignore
/// #[cgp_component]
/// trait Logger {
///     fn log(&self, message: &str);
/// }
///
/// #[cgp_component]
/// impl Logger for ConsoleLogger {
///     fn log(&self, message: &str) {
///         println!("{}", message);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn cgp_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::derive_component(attr.into(), item.into()).into()
}

/// Macro for implementing component delegation.
///
/// This macro generates implementations that delegate component functionality
/// to fields of a struct. It reduces boilerplate when implementing component
/// delegation patterns.
///
/// # Examples
///
/// ```rust,ignore
/// delegate_components! {
///     struct MyApp {
///         logger: ConsoleLogger,
///         metrics: PrometheusMetrics,
///     }
/// }
/// ```
#[proc_macro]
pub fn delegate_components(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::delegate_components(body.into())
        .unwrap()
        .into()
}

/// Macro for defining reusable component configurations.
///
/// This macro enables the creation of preset combinations of components,
/// making it easier to reuse common component configurations across
/// different types.
///
/// # Examples
///
/// ```rust,ignore
/// cgp_preset! {
///     struct LoggingPreset {
///         logger: ConsoleLogger,
///         metrics: PrometheusMetrics,
///     }
/// }
/// ```
#[proc_macro]
pub fn cgp_preset(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::define_preset(body.into())
        .unwrap()
        .into()
}

/// Utility macro for applying type replacements across multiple items.
///
/// This macro enables generating multiple variations of a code block by
/// replacing type parameters with concrete types. Useful for reducing
/// repetition when similar code is needed for different types.
///
/// # Examples
///
/// ```rust,ignore
/// for_each_replace! {
///     [T] in [String, i32, bool] {
///         fn process(value: T) -> T { value }
///     }
/// }
/// ```
#[proc_macro]
pub fn for_each_replace(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::handle_for_each_replace(body.into())
        .unwrap()
        .into()
}

/// Utility macro for replacing types with other types.
///
/// This macro provides a way to create type aliases with substituted types.
/// It's useful when working with complex generic types that need type
/// parameter substitution.
///
/// # Examples
///
/// ```rust,ignore
/// replace_with! {
///     type NewType = OldType<String>;
/// }
/// ```
#[proc_macro]
pub fn replace_with(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::handle_replace(body.into())
        .unwrap()
        .into()
}
