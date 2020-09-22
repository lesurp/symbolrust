use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn into_node(vars: &[Ident]) -> TokenStream {
    let mut acc = quote! {};
    for i in vars {
        acc = quote! {
            #acc

        impl Into<Node> for #i {
            fn into(self) -> Node {
                Node::#i(self)
            }
        }
            };
    }
    acc
}

pub(crate) fn into_boxed_node(vars: &[Ident]) -> TokenStream {
    let mut acc = quote! {};
    for i in vars {
        acc = quote! {
            #acc

        impl Into<Box<Node>> for #i {
            fn into(self) -> Box<Node> {
                Node::#i(self).into()
            }
        }
            };
    }
    acc
}
