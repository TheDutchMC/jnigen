use quote::{format_ident, quote};
use proc_macro2::{Ident, TokenStream};
use crate::codegen::{gen_jni_signature, gen_method_signature};
use crate::models::{Class, Method};

pub fn class_def(def: &Class) -> TokenStream {
    let name = def.name.split(".").collect::<Vec<_>>();
    let name = name.last().expect("Missing last element");
    let class_ident = format_ident!("{}", name);

    let struct_def = gen_struct(&class_ident);
    let methods_owned = def.methods.iter()
        .filter(|f| f.from_interface.is_none())
        .map(|f| gen_method(f))
        .collect::<Vec<_>>();

    quote! {
        #struct_def

        impl<'a> #class_ident<'a> {
            #(#methods_owned)*
        }
    }
}

fn gen_method(method: &Method) -> TokenStream {
    let method_name = gen_method_signature(method);
    if method.is_static {
        quote! {
            #method_name {
                unimplemented();
            }
        }

    } else {
        let tokens = gen_jni_call_non_static(method);
        quote! {
            #method_name {
                #tokens
            }
        }
    }
}

fn gen_jni_call_non_static(method: &Method) -> TokenStream {
    let jni_sig = gen_jni_signature(&method);
    let name = method.name.replace("[]", "");
    let jvalues = gen_jni_call_jvalues(method);
    let ret_handling = gen_jni_ret_handling(method);

    quote! {
        let result = self.env.call_method(self.inner, #name, #jni_sig, &[#jvalues])?;
        #ret_handling
    }
}

fn gen_jni_ret_handling(method: &Method) -> TokenStream {
    let ret = method.return_type.class.replace("[]", "");
    match ret.as_str() {
        "boolean" => quote!(result.z()),
        "byte" => quote!(result.b()),
        "char" => quote!(result.c()),
        "short" => quote!(result.s()),
        "int" => quote!(result.i()),
        "long" => quote!(result.j()),
        "float" => quote!(result.f()),
        "double" => quote!(result.d()),
        "void" => quote! {
            Ok(())
        },
        _ => {
            let class = if method.is_static {
                let env_ident = format_ident!("env");
                quote! {
                    let class = ejni::Class::for_name(#env_ident, #ret)?;
                }
            } else {
                quote! {
                    let class = ejni::Class::for_name(self.env, #ret)?;
                }
            };

            quote! {
                #class
                let object_raw = result.l()?;
                let object = ejni::Object::new(object_raw, class);
                Ok(object.into())
            }
        }
    }
}

fn gen_jni_call_jvalues(method: &Method) -> TokenStream {
    let jvalues = method.parameters.iter()
        .map(|f| {
            let name = f.name.as_ref().unwrap();
            let ident = format_ident!("{}", name);

            let class = f.class.replace("[]", "");
            match class.as_str() {
                "boolean" => quote!(jni::objects::JValue::Boolean(#ident)),
                "byte" => quote!(jni::objects::JValue::Byte(#ident)),
                "char" => quote!(jni::objects::JValue::Char(#ident)),
                "short" => quote!(jni::objects::JValue::Short(#ident)),
                "int" => quote!(jni::objects::JValue::Int(#ident)),
                "long" => quote!(jni::objects::JValue::Long(#ident)),
                "float" => quote!(jni::objects::JValue::Float(#ident)),
                "double" => quote!(jni::objects::JValue::Double(#ident)),
                _ => quote!(jni::JValue::Object(#ident))
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #(#jvalues),*
    }
}

fn gen_struct(ident: &Ident) -> TokenStream {
    quote! {
        pub struct #ident<'a> {
            class: ejni::Class<'a>,
            pub inner: ejni::Object<'a>,
            env: &'a jni::JNIEnv<'a>
        }

        #[allow(clippy::from_over_into)]
        impl<'a> Into<*mut jni::sys::_jobject> for #ident<'a> {
            fn into(self) -> *mut jni::sys::_jobject {
                self.inner.inner.into_inner()
            }
        }

        impl<'a> From<ejni::Object<'a>> for #ident<'a> {
            fn from(obj: ejni::Object<'a>) -> Self {
                Self {
                    inner: obj.clone(),
                    class: obj.class,
                    env: obj.env
                }
            }
        }

        impl<'a> Drop for #ident<'a> {
            fn drop(&mut self) {
                let _ = self.env.delete_local_ref(self.inner.inner);
            }
        }
    }
}