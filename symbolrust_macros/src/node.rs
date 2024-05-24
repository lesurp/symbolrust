use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn node_enum(vars: &[Ident]) -> TokenStream {
    let mut acc = quote! {};
    for i in vars {
        acc = quote! {
            #acc
            #i(#i),
        };
    }
    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub enum Node {
            #acc
        }
    }
}

// TODO: make some default impl for visitor.visit(&Node) instead of creating some weird function here!
pub(crate) fn impl_visitor_pattern(vars: &[Ident]) -> TokenStream {
    let mut acc = quote! {};

    for i in vars {
        let lower_case = i.to_string().to_lowercase();
        let fn_name = Ident::new(&format!("build_{}", lower_case), i.span());

        acc = quote! {
            #acc
            Node::#i(n) => v.#fn_name(n),
        };
    }

    quote! {
        impl Node {
            pub fn accept_visitor<T>(&self, v: &dyn Visitor<Output=T>) -> T {
                match self {
                    #acc
                }
            }
        }
    }
}
