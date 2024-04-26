use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::visit_mut::VisitMut;

#[proc_macro]
pub fn arithmetic(expression: TokenStream) -> TokenStream {
    let result = arithmetic_inner(expression.into());

    quote! {
        (|| -> Result<_, &'static str> {
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
fn arithmetic_inner(expression: TokenStream2) -> TokenStream2 {
    let mut expr: syn::Expr =
        syn::parse2(expression).expect("Failed to parse arithmetic expression");
    MutateBinOp.visit_expr_mut(&mut expr);
    expr.to_token_stream()
}

struct MutateBinOp;

impl VisitMut for MutateBinOp {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        match node {
            syn::Expr::Binary(syn::ExprBinary {
                left, right, op, ..
            }) => {
                let left_op: String = left
                    .to_token_stream()
                    .to_string()
                    .split_ascii_whitespace()
                    .collect();
                let right_op: String = right
                    .to_token_stream()
                    .to_string()
                    .split_ascii_whitespace()
                    .collect();

                self.visit_expr_mut(left);
                self.visit_expr_mut(right);

                match op {
                    syn::BinOp::Add(_) => {
                        let err = format!("{left_op} + {right_op} overflowed");
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_add(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Sub(_) => {
                        let err = format!("{left_op} - {right_op} underflowed");
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_sub(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Div(_) => {
                        let err = format!("{left_op} / {right_op} overflowed or rhs is zero");
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_div(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Mul(_) => {
                        let err = format!("{left_op} * {right_op} overflowed");
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_mul(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::Rem(_) => {
                        let err = format!("{left_op} % {right_op} overflowed or rhs is zero");
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_rem(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    syn::BinOp::BitXor(_) => {
                        let err = format!("{left_op} ^ {right_op} overflowed");
                        *node = syn::parse2::<syn::Expr>(quote! {
                            #left.checked_pow(#right).ok_or(#err)?
                        })
                        .unwrap();
                    }
                    binop => panic!("Binary operator {} not allowed", binop.to_token_stream()),
                }
            }
            syn::Expr::Unary(expr) => {
                self.visit_expr_unary_mut(expr);
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
            expr => panic!("Expression not allowed: {}", expr.to_token_stream()),
        }
    }
}
