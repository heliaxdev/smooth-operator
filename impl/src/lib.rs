use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::visit_mut::VisitMut;

/// Convert arithmetic operators within the given expression to their checked
/// variants and provide detailed error strings about which operator has failed
/// for diagnostic.
#[proc_macro]
pub fn checked(expression: TokenStream) -> TokenStream {
    let (result, expression_str) = checked_inner(expression.into());

    let crate_name = {
        let this_crate_without_impl = env!("CARGO_PKG_NAME").trim_end_matches("-impl");

        if std::env::var("CARGO_PKG_NAME").unwrap() == this_crate_without_impl {
            quote!(crate)
        } else {
            let ident = this_crate_without_impl.replace('-', "_");
            let ident = syn::Ident::new(&ident, proc_macro2::Span::call_site());
            quote!(::#ident)
        }
    };

    quote! {
        (|| -> ::core::result::Result<_, #crate_name::Error> {
            type Err = #crate_name::Error;
            const ORIGINAL_EXPR: &'static str = #expression_str;

            Ok(
                #[allow(clippy::needless_question_mark)]
                #[allow(unused_parens)]
                {
                    #result
                }
            )
        })()
    }
    .into()
}

#[inline]
fn checked_inner(expression: TokenStream2) -> (TokenStream2, String) {
    let mut expr: syn::Expr =
        syn::parse2(expression).expect("Failed to parse arithmetic expression");
    let original_expr = expr.to_token_stream().to_string();
    CheckedArith.visit_expr_mut(&mut expr);
    (expr.to_token_stream(), original_expr)
}

struct CheckedArith;

impl VisitMut for CheckedArith {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        match node {
            syn::Expr::Binary(syn::ExprBinary {
                left, right, op, ..
            }) => {
                let op_len = op.to_token_stream().to_string().len();
                let op_ix = {
                    let left_len = left.to_token_stream().to_string().len();

                    left_len
                        + 1 // Add 1 for whitespace
                        + op_len
                };

                self.visit_expr_mut(left);
                self.visit_expr_mut(right);

                match op {
                    syn::BinOp::Add(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_add(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Sub(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_sub(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Div(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_div(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Mul(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_mul(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Rem(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_rem(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    syn::BinOp::BitXor(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_pow(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Shl(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_shl(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Shr(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_shr(#right).ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_ix: #op_ix,
                                __op_len: #op_len,
                            })?
                        })
                        .unwrap();
                    }
                    _ => {}
                }
            }
            syn::Expr::Unary(syn::ExprUnary { op, expr, .. }) => {
                self.visit_expr_mut(expr);

                match op {
                    syn::UnOp::Neg(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #expr.checked_neg().ok_or(Err {
                                expr: ORIGINAL_EXPR,
                                __op_len: 1,
                                __op_ix: 0, // Negation comes first
                            })?
                        })
                        .unwrap();
                    }
                    _ => {}
                }
            }
            syn::Expr::Paren(expr) => {
                self.visit_expr_paren_mut(expr);
            }
            syn::Expr::Call(expr) => {
                self.visit_expr_call_mut(expr);
            }
            syn::Expr::MethodCall(expr) => {
                self.visit_expr_method_call_mut(expr);
            }
            syn::Expr::Path(_) | syn::Expr::Lit(_) => {}
            _ => {}
        }
    }
}
