use proc_macro::TokenStream;
use std::ops::Deref;
use quote::{quote};
use syn;
use syn::{parse_macro_input, DeriveInput, Ident, Type};
use darling;
use darling::{ast, FromDeriveInput, FromField, util};
use syn::spanned::Spanned;
use ast::Data;
use util::SpannedValue;
use crate::common::crate_name;

#[derive(Debug, Clone)]
struct Widget {
    ident: Ident,
    #[allow(dead_code)]
    ty: Type,

    x: usize,
    y: usize,
    w: usize,
    h: usize,
    #[allow(dead_code)]
    allow_intersection: bool,
}

#[derive(Debug, FromField)]
#[darling(attributes(smol_tui), and_then="map_scene_field")]
struct SceneField {
    ident: Option<Ident>,
    ty: Type,

    #[darling(default)]
    skip: bool,

    #[darling(default)]
    x: darling::util::SpannedValue<Option<usize>>,
    #[darling(default)]
    y: Option<usize>,
    #[darling(default)]
    w: Option<usize>,
    #[darling(default)]
    h: Option<usize>,

    #[darling(default)]
    allow_intersection: bool,

    // Man, this is ugly as heck
    #[darling(skip)]
    parsed: Option<util::SpannedValue<Widget>>
}

fn map_scene_field(mut f: SceneField) -> darling::Result<SceneField> {
    if f.skip {
        return Ok(f)
    }
    match (f.x.deref(), f.y, f.w, f.h) {
        (Some(x), Some(y), Some(w), Some(h)) => {
            f.parsed = Some(SpannedValue::new(Widget {
                x: *x, y, w, h,
                allow_intersection: f.allow_intersection,

                ident: f.ident.clone().unwrap(),
                ty: f.ty.clone()
            }, f.ident.span()));
            Ok(f)
        },
        _ => Err(darling::Error::
            custom("Cannot parse widget: it must have x, y, w, h attribute params specified. If you don't want the field to be used as a widget, mark it as skip")
            .with_span(&f.ident.span())) // TODO: can the error reporting be better?
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(smol_tui), supports(struct_named), and_then="map_scene_desc")]
struct SceneDesc {
    #[allow(dead_code)]
    ident: Ident,
    data: ast::Data<util::Ignored, SceneField>,

    w: usize,
    h: usize,
    char_type: Ident,
    // Man, this is ugly as heck
    #[darling(skip)]
    parsed: Option<Vec<util::SpannedValue<Widget>>>
}

fn map_scene_desc(mut desc: SceneDesc) -> darling::Result<SceneDesc> {
    let mut widgets = vec![];

    match desc.data {
        Data::Struct(ref struc) => {
            for f in struc.iter() {
                if let Some(widget) = &f.parsed {
                    widgets.push(widget.clone())
                }
            }
        }
        _ => return Err(darling::Error::custom("Only structs may be scenes"))
    }
    // TODO: validate intersections and stuff

    desc.parsed = Some(widgets);

    Ok(desc)
}

pub fn scene_derive_impl(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input: DeriveInput = parse_macro_input!(input);

    let name = &input.ident;

    let inp: SceneDesc = match FromDeriveInput::from_derive_input(&input) {
        Ok(v) => v,
        // TODO: how to combine darling with proc_macro_error?
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let widgets = inp.parsed.unwrap();

    let crate_name = crate_name();

    let widget_renders = widgets.into_iter().map(|widget| {
        let id = &widget.ident;
        // we may have used quote_spanned!, but it produces quite misleading errors (and hints!)
        // maybe we are better if w/o span info
        let x = widget.x;
        let y = widget.y;
        let w = widget.w;
        let h = widget.h;
        // scary, but does roughly the following:
        // 1. subspan the frame (unsafely, as validation is all done during parsing)
        // 2. convert the fixed frame to a size-erased one (if needed)
        quote! {
            #crate_name::FixedWidget::render_fixed(&self.#id,
                &mut frame.subframe::<#x, #y, #w, #h>(), tick);
        }
    });

    let t = inp.char_type;
    let w = inp.w;
    let h = inp.h;

    let gen = quote! {
        impl #crate_name::Scene<#t, #w, #h> for #name {
            fn render(&self, frame: &mut #crate_name::FixedFrameAccessor<#t, #w, #h>, tick: u32) {
                #(#widget_renders)*
            }
        }
    };
    gen.into()
}