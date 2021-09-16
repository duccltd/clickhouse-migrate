extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn use_migrations(_input: TokenStream) -> TokenStream {
    unimplemented("used to load in migrations at compile time")
}
