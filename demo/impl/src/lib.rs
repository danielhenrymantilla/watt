use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro2::proc_macro_derive(Demo)]
pub fn demo(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse2(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error(),
    };

    let ident = input.ident;
    let message = format!("Hello from WASM! My name is {}.", ident);

    quote! {
        impl #ident {
            pub const MESSAGE: &'static str = #message;
        }
    }
}
