use crate::{LanguageOp, Operator};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

// HERE BE DRAGONS? Refactor this whole shite
// (do I even want / need this..? If I'm always using some cli + dsl anyway, rather than rust's operators...)

pub(crate) fn all_ops(operators: &[Operator]) -> TokenStream {
    let all_variants: Vec<Ident> = operators.iter().map(|o| o.ident.clone()).collect();

    let binary_ops = operators
        .iter()
        .enumerate()
        .filter_map(|(i, op)| {
            if let Some(language_ops) = &op.language_ops {
                Some((&op.ident, language_ops, i))
            } else {
                None
            }
        })
        .fold(quote! {}, |q, (op_wrapper, language_ops, i)| {
            let mut all_variants = all_variants.clone();
            all_variants.remove(i);
            let mut next = quote! {};
            for language_op in language_ops {
                let next2 = single_op(all_variants.clone(), language_op, &op_wrapper);
                next = quote! {
                    #next
                   #next2
                };
            }
            quote! {
                #q
                #next
            }
        });

    // FIXME: so half of those macros try to be generic, but here I am hardcoding the negation
    // variant...
    let mut unary_neg = quote! {
        impl std::ops::Neg for Node {
            type Output = Node;
            fn neg(self) -> Self::Output {
                Negation::new(self).into()
            }
        }
    };

    for var in all_variants {
        if var == "Negation" {
            unary_neg = quote! {
                #unary_neg
                impl std::ops::Neg for #var {
                    type Output = Node;
                    fn neg(self) -> Self::Output {
                        *self.val
                    }
                }
            };
        } else {
            unary_neg = quote! {
                #unary_neg
                impl std::ops::Neg for #var {
                    type Output = Node;
                    fn neg(self) -> Self::Output {
                        Negation::new(self).into()
                    }
                }
            };
        }
    }
    quote! {
        #binary_ops
        #unary_neg
    }
}

fn single_op(
    mut all_variants: Vec<Ident>,
    language_op: &LanguageOp,
    op_wrapper: &Ident,
) -> TokenStream {
    let operator = &language_op.name;
    let inverse = language_op.inverse;

    all_variants.push(Ident::new("Node", Span::call_site()));

    let ct: Vec<_> = vec!["f64", "i64"]
        .into_iter()
        .map(|name| Ident::new(name, Span::call_site()))
        .collect();

    let fn_name = Ident::new(&operator.to_string().to_lowercase(), operator.span());

    let mut acc = quote! {};

    // LHS = scalar/vars, RHS = vars
    for lhs in ct.iter().chain(all_variants.iter()) {
        for rhs in all_variants.iter() {
            acc = quote! {
                    #acc
                    impl std::ops::#operator<#rhs> for #lhs {
                        type Output = Node;
                        fn #fn_name(self, rhs: #rhs) -> Self::Output {
                            #op_wrapper::from_binary::<_, _, #inverse>(self, rhs).into()
                        }
                    }

                    impl std::ops::#operator<#rhs> for &#lhs {
                        type Output = Node;
                        fn #fn_name(self, rhs: #rhs) -> Self::Output {
                            #op_wrapper::from_binary::<_, _, #inverse>(self.clone(), rhs).into()
                        }
                    }

                    impl std::ops::#operator<&#rhs> for #lhs {
                        type Output = Node;
                        fn #fn_name(self, rhs: &#rhs) -> Self::Output {
                            #op_wrapper::from_binary::<_, _, #inverse>(self, rhs.clone()).into()
                        }
                    }

                    impl std::ops::#operator<&#rhs> for &#lhs {
                        type Output = Node;
                        fn #fn_name(self, rhs: &#rhs) -> Self::Output {
                            #op_wrapper::from_binary::<_, _, #inverse>(self.clone(), rhs.clone()).into()
                        }
                    }
            };
        }
    }

    // RHS = scalar, LHS = all vars but op
    for lhs in all_variants.iter() {
        for rhs in ct.iter() {
            acc = quote! {
                #acc
                impl std::ops::#operator<#rhs> for #lhs {
                    type Output = Node;
                    fn #fn_name(self, rhs: #rhs) -> Self::Output {
                        #op_wrapper::from_binary::<_, _, #inverse>(self, rhs).into()
                    }
                }

                impl std::ops::#operator<#rhs> for &#lhs {
                    type Output = Node;
                    fn #fn_name(self, rhs: #rhs) -> Self::Output {
                        #op_wrapper::from_binary::<_, _, #inverse>(self.clone(), rhs).into()
                    }
                }
            };
        }
    }

    // LHS = op, RHS = scalar/vars
    for rhs in ct.iter().chain(all_variants.iter()) {
        acc = quote! {
            #acc
        impl std::ops::#operator<#rhs> for #op_wrapper {
            type Output = Node;
            fn #fn_name(self, rhs: #rhs) -> Self::Output {
                self.append::<_, #inverse>(rhs).into()
            }
        }

        impl std::ops::#operator<#rhs> for &#op_wrapper {
            type Output = Node;
            fn #fn_name(self, rhs: #rhs) -> Self::Output {
                self.clone().append::<_, #inverse>(rhs).into()
            }
        }
        };
    }

    // LHS = RHS = op
    acc = quote! {
            #acc
            impl std::ops::#operator<#op_wrapper> for #op_wrapper {
                type Output = Node;
                fn #fn_name(self, rhs: #op_wrapper) -> Self::Output {
                    #op_wrapper::fuse::<#inverse>(self, rhs).into()
                }
            }

            impl std::ops::#operator<#op_wrapper> for &#op_wrapper {
                type Output = Node;
                fn #fn_name(self, rhs: #op_wrapper) -> Self::Output {
                    #op_wrapper::fuse::<#inverse>(self.clone(), rhs).into()
                }
            }

            impl std::ops::#operator<&#op_wrapper> for #op_wrapper {
                type Output = Node;
                fn #fn_name(self, rhs: &#op_wrapper) -> Self::Output {
                    #op_wrapper::fuse::<#inverse>(self, rhs.clone()).into()
                }
            }

            impl std::ops::#operator<&#op_wrapper> for &#op_wrapper {
                type Output = Node;
                fn #fn_name(self, rhs: &#op_wrapper) -> Self::Output {
                    #op_wrapper::fuse::<#inverse>(self.clone(), rhs.clone()).into()
                }
            }
    };

    // Generate *Assign operators
    // LHS *has* to be a node (self-assign can't change the type of the value)
    let assign_op = quote::format_ident!("{}Assign", operator);
    let assign_fn_name = quote::format_ident!("{}_assign", fn_name);

    // last element is the "Node" which we *need* to implement manually
    // so we pop it's corresponding ident here
    all_variants.pop();

    // RHS = var/scalar
    for rhs in ct.iter().chain(all_variants.iter()) {
        acc = quote! {
                #acc
                impl std::ops::#assign_op<#rhs> for Node {
                    fn #assign_fn_name(&mut self, rhs: #rhs)  {
                        let fles = std::mem::replace(self, Constant::new(0).into());
                        *self = match fles {
                            Node::#op_wrapper(lhs) =>   lhs.append::<_, #inverse>(rhs) ,
                            lhs =>   { #op_wrapper::from_binary::<_, _, #inverse>(lhs, rhs) },
                        }.into();
                    }
                }

                impl std::ops::#assign_op<&#rhs> for Node {
                    fn #assign_fn_name(&mut self, rhs: &#rhs)  {
                        let fles = std::mem::replace(self, Constant::new(0).into());
                        *self = match fles {
                            Node::#op_wrapper(lhs) =>   lhs.append::<_, #inverse>(rhs.clone()) ,
                            lhs =>   { #op_wrapper::from_binary::<_, _, #inverse>(lhs, rhs.clone()) },
                        }.into();
                    }
                }
        };
    }

    // RHS = op_wrapper (so we don't add additions, but instead get a single flat addition)
    acc = quote! {
            #acc
            impl std::ops::#assign_op<#op_wrapper> for Node {
                fn #assign_fn_name(&mut self, rhs: #op_wrapper)  {
                    let fles = std::mem::replace(self, Constant::new(0).into());
                    *self = match fles {
                        Node::#op_wrapper(lhs) =>  #op_wrapper::fuse::<#inverse>(lhs, rhs),
                        lhs =>    rhs.prepend::<_, #inverse>(lhs) ,
                    }.into();
                }
            }
    };

    // RHS = Node, the most generic one
    acc = quote! {
            #acc
            impl std::ops::#assign_op<Node> for Node {
                fn #assign_fn_name(&mut self, rhs: Node)  {
                    let fles = std::mem::replace(self, Constant::new(0).into());
                    *self = match (fles, rhs) {
                        (Node::#op_wrapper(lhs), Node::#op_wrapper(rhs)) =>  #op_wrapper::fuse::<#inverse>(lhs, rhs),
                        (Node::#op_wrapper(lhs), rhs) => lhs.append::<_, #inverse>(rhs),
                        (lhs, Node::#op_wrapper(rhs)) =>   rhs.prepend::<_, #inverse>(lhs),
                        (lhs, rhs) => {  #op_wrapper::from_binary::<_, _, #inverse>(lhs, rhs)},
                    }.into();
                }
            }
    };

    acc
}
