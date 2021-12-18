
use proc_macro2::Span;
use syn::{Ident, Error};

pub fn crate_name() -> Ident {
    let name = "smol-tui";
	proc_macro_crate::crate_name(name)
		.map(|name| {
            match name {
                proc_macro_crate::FoundCrate::Itself => Ident::new("crate", Span::call_site()),
                proc_macro_crate::FoundCrate::Name(name) => 
                    Ident::new(&name, Span::call_site()),
            }
        })
		.map_err(|e| Error::new(Span::call_site(), &e))
        .unwrap_or_else(|e| panic!("Cannot determine {} crate name: {}", name, e))
}