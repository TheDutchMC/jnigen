use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::ejni_types::EJNIType;
use crate::models::{Method, Parameter};

pub mod class;
pub mod interface;
pub mod mod_file;

pub fn gen_jni_signature(method: &Method) -> String {
    let return_type = method.return_type.class.replace("[]", "");
    let return_type_jni = jni_type(&return_type, method.return_type.is_array);

    let params = method.parameters.iter()
        .map(|f| jni_type(f.class.replace("[]", ""), f.is_array))
        .collect::<Vec<_>>();

    format!("({}){}", params.join(""), return_type_jni)
}

fn jni_type<S: AsRef<str>>(ty: S, array: bool) -> String {
    let ident = match ty.as_ref() {
        "void" => "V".to_string(),
        "byte" => "B".to_string(),
        "int" => "I".to_string(),
        "long" => "J".to_string(),
        "boolean" => "Z".to_string(),
        "char" => "C".to_string(),
        "short" => "S".to_string(),
        "float" => "F".to_string(),
        "double" => "D".to_string(),
        _ => format!("L{};", ty.as_ref().replace(".", "/"))
    };

    if array {
        format!("[{}", ident)
    } else {
        ident
    }
}

pub fn gen_method_signature(method: &Method) -> TokenStream {
    let mut params = Vec::new();

    // If the method is static, a reference to a JNIEnv must be given
    // If it is non-static, &self must be given
    if method.is_static {
        params.push(gen_env_param())
    } else {
        params.push(quote! (&self))
    }

    // Method parameters themselves
    params.append(&mut method.parameters.iter()
        .map(|f| param_to_tokens(&f, false))
        .collect::<Vec<_>>());

    let return_type = param_to_tokens(&method.return_type, true);
    let wrapped_return_type = wrap_result(return_type);

    let method_ident = format_ident!("{}", method.name.to_case(Case::Snake));
    quote! {
        fn #method_ident(#(#params),*) -> #wrapped_return_type
    }
}

fn gen_env_param() -> TokenStream {
    let ident = format_ident!("env");
    quote! {
        #ident: &'a jni::JNIEnv<'a>
    }
}

fn param_to_tokens(parameter: &Parameter, ret: bool) -> TokenStream {
    let class_name = class_name_to_tokens(&parameter.class);
    let class_name = if parameter.is_array {
        quote!(std::vec::Vec<#class_name>)
    } else {
        class_name
    };

    if ret {
        class_name
    } else {
        let name = parameter.name.as_ref().unwrap(); // Safe, name is present for method parameters
        let ident = format_ident!("{}", name);
        quote! ( #ident: #class_name)
    }
}

fn wrap_result(i: TokenStream) -> TokenStream {
    quote! { jni::errors::Result<#i> }
}

fn class_name_to_tokens(class: &str) -> TokenStream {
    let class = class.replace("[]", "");
    match class.as_str() {
        "void" => quote!(()),
        "int" => quote!(i32),
        "long" => quote!(i64),
        "float" => quote!(f32),
        "double" => quote!(f64),
        "short" => quote!(i16),
        "char" => quote!(u16),
        "byte" => quote!(u8),
        "boolean" => quote!(bool),
        _ => {
            match EJNIType::from_str(&class) {
                Some(e) => e.to_tokens(),
                None => {
                    let rustified = class.replace(".", "::");
                    let rustified: TokenStream = rustified.parse().expect("Unable to parse tokens");
                    quote!(#rustified)
                }
            }
        }
    }
}