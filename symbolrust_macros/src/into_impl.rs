use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn into_node(vars: &[Ident]) -> TokenStream {
    let mut acc = quote! {};
    for i in vars {
        acc = quote! {
            #acc

            impl From<#i> for Node {
                fn from(i: #i) -> Node {
                    Node::#i(i)
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

            impl From<#i> for Box<Node> {
                fn from(i: #i) -> Box<Node> {
                    Node::#i(i).into()
                }
            }
        };
    }
    acc
}
