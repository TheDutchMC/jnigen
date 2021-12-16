use std::path::Path;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use anyhow::Result;

pub fn gen_mod(path: &Path) -> Result<TokenStream> {
    let mod_tokens = get_submods(path)?.iter()
        .map(|f| gen_entry(f))
        .collect::<Vec<_>>();

    Ok(quote! {
        #(#mod_tokens)*
    })
}

fn gen_entry(entry: &Entry) -> TokenStream {
    let ident = format_ident!("{}", entry.name);
    if entry.file {
        quote! {
            mod #ident;
            pub use #ident::*;
        }
    } else {
        quote! {
            pub mod #ident;
        }
    }
}

struct Entry {
    name: String,
    file: bool
}

fn get_submods(path: &Path) -> Result<Vec<Entry>> {
    fs::read_dir(&path)?.into_iter()
        .map(|f| {
            let f = f?;
            Ok(Entry {
                name: f.file_name().to_str().unwrap().to_string().replace(".rs", ""),
                file: f.file_type()?.is_file()
            })
        })
        .collect::<Result<Vec<_>>>()
}