use crate::Operator;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn all_precedence(operators: &[Operator]) -> TokenStream {
    let precedence_impl = operators.iter().fold(quote! {}, |q, operator| {
        let variant_name = &operator.ident;
        let imp = if let Some(precedence) = operator.precedence {
            quote! {
                fn precedence(&self) -> Option<u32> { Some(#precedence) }
            }
        } else {
            quote! {}
        };
        quote! {
            #q
            impl Precedence for #variant_name {
                #imp
            }
        }
    });

    let node_impl_matches = operators.iter().fold(quote! {}, |q, op| {
        let var = &op.ident;
        quote! {
            #q
            Node::#var(n) => n.precedence(),
        }
    });

    quote! {
        pub trait Precedence {
            fn precedence(&self) -> Option<u32> { None }
        }

        impl Precedence for Node {
            fn precedence(&self) -> Option<u32> {
                match self {
                    #node_impl_matches
                }
            }
        }

        #precedence_impl
    }
}
