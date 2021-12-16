use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use crate::codegen::gen_method_signature;
use crate::models::Class;

pub fn gen_interface(def: &Class) -> TokenStream {
    let name = def.name.split(".").collect::<Vec<_>>();
    let name = name.last().expect("Missing last element");
    let name_ident = format_ident!("{}", name);

    let methods = def.methods.iter()
        .map(|f| {
            let method = gen_method_signature(&f);
            quote! ( #method; )
        })
        .collect::<Vec<_>>();

    quote! {
        pub trait #name_ident<'a> {
            #(#methods)*
        }
    }
}