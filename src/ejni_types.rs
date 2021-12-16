use proc_macro2::TokenStream;
use quote::quote;

pub enum EJNIType {
    Iterator,
    List,
    Map,
    Set,
    String,
}

impl EJNIType {
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        match s.as_ref() {
            "java.util.Iterator" => Some(Self::Iterator),
            "java.util.List" => Some(Self::List),
            "java.util.ArrayList" => Some(Self::List),
            "java.util.Map" => Some(Self::Map),
            "java.util.HashMap" => Some(Self::Map),
            "java.util.Set" => Some(Self::Set),
            "java.util.HashSet" => Some(Self::Set),
            "java.lang.String" => Some(Self::String),
            _ => None
        }
    }

    pub fn to_tokens(&self) -> TokenStream {
        match self {
            Self::Iterator => quote!(ejni::Iterator),
            Self::Map => quote!(ejni::Map),
            Self::List => quote!(ejni::List),
            Self::Set => quote!(ejni::Set),
            Self::String => quote!(ejni::JavaString),
        }
    }
}