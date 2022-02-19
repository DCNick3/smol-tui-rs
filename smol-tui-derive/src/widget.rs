use syn::{parse_macro_input, ItemImpl, GenericArgument};
use quote::{quote};
use crate::common::crate_name;

use proc_macro::TokenStream as TokenStream1;

pub fn fixed_widget_adapter_impl(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as ItemImpl);
    
    //println!("{:#?}", input);
    
    let (bang, path, _) = input.trait_.clone().unwrap_or_else(|| panic!("You should put this attribute only on Widget trait implementations"));
    if let Some(_) = bang {
        panic!("Bang implementations are not supported")
    }
    
    let ty = input.self_ty.as_ref();

    // I don't wanna do the real parsing here
    // so just check whether there are __any__ generic parameters =)
    let generic = !input.generics.params.empty_or_trailing();

    let element_type = match path.segments.last().unwrap_or_else(|| panic!("bad2")).arguments {
        //syn::PathArguments::None => (true,),
        syn::PathArguments::AngleBracketed(ref bb) => {
            if bb.args.len() != 1 {
                panic!("Something weird, can't parse it")
            }
            if let GenericArgument::Type(ref t) = bb.args[0] {
                t
            } else {
                panic!("Something weird, can't parse it")
            }
        },
        _ => panic!("Something weird, can't parse it"),
    };
    
    let crate_name = crate_name();

    let gen = if generic {
        quote! {
            #input

            impl<T, const W: usize, const H: usize> FixedWidget<T, W, H> for #ty
            where
                T: Copy,
                T: DefaultFill
            {
                type State = <#ty as #crate_name::Widget<T>>::State;

                fn render_fixed(&self, state: &Self::State, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32) {
                    #crate_name::Widget::render(self, state, &mut #crate_name::FrameAccessor::<T>::from(frame), tick)
                }
            }
        }
    } else {
        quote! {
            #input

            impl<const W: usize, const H: usize> FixedWidget<#element_type, W, H> for #ty
            where
                T: Copy,
                T: DefaultFill
            {
                type State = <#ty as #crate_name::Widget<#element_type>>::State;

                fn render_fixed(&self, state: &Self::State, frame: &mut FixedFrameAccessor<#element_type, W, H>, tick: u32) {
                    #crate_name::Widget::render(self, state, &mut #crate_name::FrameAccessor::<#element_type>::from(frame), tick)
                }
            }
        }
    };
    gen.into()
}