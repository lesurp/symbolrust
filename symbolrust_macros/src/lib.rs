mod into_impl;
mod node;
mod operators;
mod precedence;
mod visitor_trait;

use quote::quote;
use syn::parse::Parser;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Result, Token};

struct LanguageOp {
    name: Ident,
    // whether or not the given op is the opposite operation of the canonical one
    // e.g. subtraction for addition, or division for multiplication
    // defaults to false if not specified in the macro generation
    inverse: bool,
}

impl LanguageOp {
    fn from_elem(elem: &syn::Expr) -> Self {
        match elem {
            syn::Expr::Path(op_only) => {
                let name = op_only.path.get_ident().expect("Expected an ident").clone();
                LanguageOp {
                    name,
                    inverse: false,
                }
            }
            syn::Expr::Struct(estr) => {
                let mut inverse = false;
                let name = estr.path.get_ident().expect("Expected an ident").clone();
                for field in &estr.fields {
                    match &field.member {
                        syn::Member::Unnamed(_) => panic!("Wrong parameter"),
                        syn::Member::Named(n) => {
                            if &n.to_string() == "inverse" {
                                match &field.expr {
                                    syn::Expr::Lit(l) => match &l.lit {
                                        syn::Lit::Bool(i) => inverse = i.value,
                                        _ => panic!("Expected boolean"),
                                    },
                                    _ => panic!("Wrong type, expected boolean"),
                                }
                            } else {
                                panic!(
                                    "Unexpected token in LanguageOp definition: {}",
                                    n.to_string()
                                );
                            }
                        }
                    }
                }

                LanguageOp { name, inverse }
            }
            _ => panic!("Expecting an ExprStruct in LanguageOp definition"),
        }
    }
}

struct Operator {
    pub(crate) ident: Ident,
    pub(crate) precedence: Option<u32>,
    pub(crate) language_ops: Option<Vec<LanguageOp>>,
}

impl Parse for Operator {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident;
        let mut precedence = None;
        let mut language_ops = None;
        if input.peek2(syn::token::Brace) {
            let expr: syn::ExprStruct = input.parse()?;
            ident = expr.path.get_ident().expect("Expected an ident").clone();
            for field in expr.fields {
                match field.member {
                    syn::Member::Unnamed(_) => panic!("Wrong parameter"),
                    syn::Member::Named(n) => {
                        if &n.to_string() == "precedence" {
                            match field.expr {
                                syn::Expr::Lit(l) => match l.lit {
                                    syn::Lit::Int(i) => {
                                        precedence = Some(i.base10_parse().expect("Expected u32"))
                                    }
                                    _ => panic!("Expected u32"),
                                },
                                _ => panic!("Wrong type, expected u32"),
                            }
                        } else if &n.to_string() == "language_ops" {
                            match field.expr {
                                syn::Expr::Array(arr) => {
                                    language_ops = Some(
                                        arr.elems
                                            .iter()
                                            .map(|elem| LanguageOp::from_elem(elem))
                                            .collect(),
                                    );
                                }
                                syn::Expr::Path(op_only) => {
                                    language_ops = Some(vec![LanguageOp {
                                        name: op_only
                                            .path
                                            .get_ident()
                                            .expect("Expected an ident")
                                            .clone(),
                                        inverse: false,
                                    }]);
                                }
                                _ => panic!("Wrong type, expected operator trait"),
                            }
                        }
                    }
                }
            }
        } else {
            ident = input.parse()?;
        }

        Ok(Operator {
            ident,
            precedence,
            language_ops,
        })
    }
}

fn boiler_plate_chef_proc2(input: proc_macro::TokenStream) -> proc_macro2::TokenStream {
    type Vars = Punctuated<Operator, Token![,]>;

    let parser = Vars::parse_terminated;
    let vars: Vec<_> = parser
        .parse(input)
        .expect("Could not parse operators")
        .into_iter()
        .collect();

    let vars_ident: Vec<Ident> = vars.iter().map(|o| o.ident.clone()).collect();

    let node_enum = node::node_enum(&vars_ident);
    let node_impl = node::impl_visitor_pattern(&vars_ident);
    let into_node = into_impl::into_node(&vars_ident);
    let into_boxed = into_impl::into_boxed_node(&vars_ident);
    let visitor_trait = visitor_trait::visitor_trait(&vars_ident);
    let operators = operators::all_ops(&vars);
    let precedence = precedence::all_precedence(&vars);

    let out = quote! {
        #node_enum
        #into_node
        #node_impl
        #into_boxed
        #visitor_trait
        #operators
        #precedence
    };

    out
}

#[proc_macro]
pub fn boiler_plate_chef(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    boiler_plate_chef_proc2(input).into()
}
