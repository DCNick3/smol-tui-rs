use crate::common::crate_name;
use darling::{self, FromAttributes, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_quote, Attribute, AttributeArgs, Expr};
use syn::{parse_macro_input, Ident, Type};

#[derive(Debug, FromAttributes)]
#[darling(attributes(smol_tui), allow_unknown_fields)]
struct SkipField {
    #[darling(default)]
    skip: bool,

    #[darling(default)]
    init: Option<String>, // rrr, we pass an expression as string. this kinda sucks
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(smol_tui))]
struct WidgetAttrs {
    x: usize,
    y: usize,
    w: usize,
    h: usize,

    #[darling(default)]
    #[allow(dead_code)]
    allow_intersection: bool,
}

#[derive(Debug, FromMeta)]
struct SceneAttr {
    w: usize,
    h: usize,
    char_type: Ident,
}

pub fn smol_tui_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let mut input: syn::ItemStruct = parse_macro_input!(input);
    let args: AttributeArgs = parse_macro_input!(args);
    let scene_attr = match SceneAttr::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let crate_name = crate_name();

    let mut inits = vec![];

    for f in input.fields.iter_mut() {
        let mut my_attrs: Vec<Attribute> = vec![];

        let field = f.ident.clone().unwrap();

        f.attrs.retain(|attr| {
            if let Some(ident) = attr.path.get_ident() {
                if ident.to_string() == "smol_tui" {
                    my_attrs.push(attr.clone());

                    return false;
                }
            }
            true
        });

        let skip = SkipField::from_attributes(&my_attrs).unwrap();
        if skip.skip {
            let init = if let Some(ref init) = skip.init {
                let init: Expr = syn::parse_str(init).unwrap();
                quote! { #init }
            } else {
                quote! { Default::default() }
            };
            inits.push(quote! { #field : #init, });
            continue;
        }

        let widget_attrs = match WidgetAttrs::from_attributes(&my_attrs) {
            Ok(a) => a,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            } // TODO: accumulate all the errors
        };

        let SceneAttr {
            w: scene_w,
            h: scene_h,
            ref char_type,
        } = scene_attr;

        let WidgetAttrs { x, y, w, h, .. } = widget_attrs;

        let ty = &f.ty;
        let new_ty: Type = parse_quote! {
            #crate_name::widgets::Subframer::<#char_type, #ty, #scene_w, #scene_h, #x, #y, #w, #h>
        };
        f.ty = new_ty;

        inits.push(quote! { #field : Default::default(), });

        // TODO: check widget intersections
    }

    let name = &input.ident;

    let init_impl = quote! {
        impl ::core::default::Default for #name {
            fn default() -> Self {
                Self {
                    #(#inits)*
                }
            }
        }
    };

    let gen = quote! {
        #input
        #init_impl
    };
    gen.into()
}
