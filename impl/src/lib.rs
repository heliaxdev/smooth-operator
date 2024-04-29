use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::visit_mut::VisitMut;

/// Convert arithmetic operators within the given expression to their checked
/// variants and provide detailed error strings about which operator has failed
/// for diagnostic.
#[proc_macro]
pub fn checked(expression: TokenStream) -> TokenStream {
    let result = checked_inner(expression.into());

    quote! {
        (|| -> std::result::Result<_, String> {
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
fn checked_inner(expression: TokenStream2) -> TokenStream2 {
    let mut expr: syn::Expr =
        syn::parse2(expression).expect("Failed to parse arithmetic expression");
    CheckedArith.visit_expr_mut(&mut expr);
    expr.to_token_stream()
}

struct CheckedArith;

impl VisitMut for CheckedArith {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        match node {
            syn::Expr::Binary(expr_binary) => {
                let original_expr: String = expr_binary.to_token_stream().to_string();

                let syn::ExprBinary {
                    left, right, op, ..
                } = expr_binary;

                let left_len: usize = left.to_token_stream().to_string().len();

                let err = Error {
                    expr: original_expr,
                    op_ix: left_len + 2, // Add 1 for whitespace and 1 for next char
                }
                .to_string();

                self.visit_expr_mut(left);
                self.visit_expr_mut(right);

                match op {
                    syn::BinOp::Add(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_add(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Sub(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_sub(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Div(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_div(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Mul(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_mul(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Rem(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_rem(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::BitXor(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_pow(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Shl(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_shl(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Shr(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_shr(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    _ => {}
                }
            }
            syn::Expr::Unary(expr_unary) => {
                let original_expr: String = expr_unary.to_token_stream().to_string();

                let syn::ExprUnary { op, expr, .. } = expr_unary;

                let err = Error {
                    expr: original_expr,
                    op_ix: 0, // Negation comes first
                }
                .to_string();

                self.visit_expr_mut(expr);

                match op {
                    syn::UnOp::Neg(_) => {
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #expr.checked_neg().ok_or(#err)?
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

/// Checked arithmetics error
#[derive(Debug)]
struct Error {
    /// The original expression given to [`checked`] macro.
    pub expr: String,
    /// Index of the operator that has failed within the `expr`.
    pub op_ix: usize,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Error { expr, op_ix } = self;
        let (prefix, rest) = expr.split_at(op_ix.checked_sub(1).unwrap_or_default());
        let (op, suffix) = rest.split_at(1);
        write!(f, "Failure in: {prefix} 》{op}《 {suffix}")
    }
}
