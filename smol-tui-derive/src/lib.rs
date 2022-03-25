use proc_macro::TokenStream;

mod common;
mod scene;
mod widget;

#[proc_macro_attribute]
pub fn smol_tui_scene(args: TokenStream, input: TokenStream) -> TokenStream {
    scene::smol_tui_impl(args, input)
}

#[proc_macro_attribute]
pub fn fixed_widget_adapter(_args: TokenStream, input: TokenStream) -> TokenStream {
    widget::fixed_widget_adapter_impl(input)
}
