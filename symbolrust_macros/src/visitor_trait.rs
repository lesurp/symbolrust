use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn visitor_trait(vars: &[Ident]) -> TokenStream {
    let mut acc = quote! {};
    for i in vars {
        let lower_case = i.to_string().to_lowercase();
        let fn_name = Ident::new(&format!("build_{}", lower_case), i.span());

        acc = quote! {
            #acc
            fn #fn_name(&self, n: &#i) -> Self::Output;
        };
    }

    quote! {
        pub trait Visitor {
            type Output;
            #acc
        }
    }
}
