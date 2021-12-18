use proc_macro::TokenStream;

mod common;
mod scene;
mod widget;

#[proc_macro_derive(Scene, attributes(smol_tui))]
pub fn scene_derive(input: TokenStream) -> TokenStream {
    scene::scene_derive_impl(input)
}

#[proc_macro_attribute]
pub fn fixed_widget_adapter(_args: TokenStream, input: TokenStream) -> TokenStream {
    widget::fixed_widget_adapter_impl(input)
}