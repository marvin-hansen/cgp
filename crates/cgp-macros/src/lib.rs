extern crate proc_macro;

mod helper;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::helper::derive::derive_component(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn strip_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    crate::helper::strip_async::strip_async(stream.into()).into()
}
