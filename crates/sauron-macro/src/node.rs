use proc_macro2::TokenStream;
use quote::quote;
use rstml::node::{KeyedAttributeValue, Node, NodeAttribute, NodeBlock};
use sauron_core::html::lookup;
use syn::{Expr, ExprForLoop, Stmt, ExprLit};

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    match rstml::parse(input) {
        Ok(nodes) => {
            let (nodes_exprs, ts) = multiple_nodes(nodes);
            dbg!(nodes_exprs);
            ts
        }
        Err(error) => error.to_compile_error(),
    }
}

fn multiple_nodes(mut nodes: Vec<Node>) -> (Vec<DiffExpr>, TokenStream) {
    let only_one_node = nodes.len() == 1;
    if only_one_node {
        let (diff_expr, node_tokens) = single_node(nodes.remove(0));
        (vec![diff_expr], quote! {
            #node_tokens
        })
    } else {
        let (nodes_diff_exprs, children_tokens) = nodes_to_tokens(nodes);
        (nodes_diff_exprs, quote! {
            sauron::html::node_list([
                #children_tokens
            ])
        })
    }
}

fn single_node(node: Node) -> (DiffExpr, TokenStream) {
    match node {
        Node::Element(elm) => {
            let open_tag = elm.open_tag;
            let tag = open_tag.name.to_string();

            let self_closing = lookup::is_self_closing(&tag);
            let namespace = lookup::tag_namespace(&tag);
            let (diff_vars, attributes) = node_attributes(open_tag.attributes);
            let (children_diff_vars, children) = nodes_to_tokens(elm.children);
            if let Some(namespace) = namespace {
                (diff_expr(diff_vars, children_diff_vars), quote! {
                    sauron::html::element_ns(Some(#namespace), #tag, [#attributes], [#children], #self_closing)
                })
            } else {
                (diff_expr(diff_vars, children_diff_vars), quote! {
                    sauron::html::element_ns(None, #tag, [#attributes], [#children], #self_closing)
                })
            }
        }
        Node::Fragment(fragment) => {
            let (fragment_diff_vars, nodes) = multiple_nodes(fragment.children);
            (diff_expr(vec![], fragment_diff_vars), nodes)
        }
        Node::Text(node_text) => {
            let text = node_text.value_string();
            (DiffExpr::none(), quote! {
                sauron::html::text(#text)
            })
        }
        Node::RawText(raw_text) => {
            let text = raw_text.to_token_stream_string();
            (DiffExpr::none(), quote! {
                sauron::html::text(#text)
            })
        }
        Node::Comment(comment) => {
            let comment_text = comment.value.value();
            (DiffExpr::none(),quote! {
                sauron::html::comment(#comment_text)
            })
        }
        Node::Doctype(doctype) => {
            let value = doctype.value.to_token_stream_string();
            (DiffExpr::none(),quote! {
                sauron::html::doctype(#value)
            })
        }
        Node::Block(block) => match block {
            NodeBlock::Invalid { .. } => {
                (DiffExpr::none(),quote! {
                    compile_error!("invalid block: {:?}", #block);
                })
            }
            NodeBlock::ValidBlock(block) => {
                if let Some(ExprForLoop { pat, expr, body, .. })= braced_for_loop(&block) {
                    (diff_expr(vec![expr.as_ref().clone()], vec![]),quote! {
                        {
                            let mut receiver = vec![];
                            for #pat in #expr {
                                #[allow(unused_braces)]
                                receiver.push(#body)
                            }
                            sauron::html::node_list(receiver)
                        }
                    })
                } else if let Some(lit) = lit_expr(&block) {
                    (DiffExpr::none(),quote!{
                        #lit
                    })
                } else if let Some(expr) = some_expr(&block) {
                    (diff_expr(vec![expr.clone()],vec![]), quote!{
                        #expr
                    })
                } else {
                    (DiffExpr::none(), quote! {
                        #block
                    })
                }
            }
        },
    }
}

#[derive(Debug)]
struct DiffExpr{
    expr: Vec<Expr>,
    children: Vec<DiffExpr>,
}

impl DiffExpr{
    fn none() -> Self {
        Self{
            expr: vec![],
            children: vec![],
        }
    }
}

fn diff_expr(expr: Vec<Expr>, children: Vec<DiffExpr>) -> DiffExpr{
    DiffExpr{
        expr,
        children
    }
}

fn nodes_to_tokens(nodes: Vec<Node>) -> (Vec<DiffExpr>, TokenStream) {
    let mut tokens = TokenStream::new();
    let mut nodes_diff_var =vec![];
    for node in nodes {
        let (diff_vars, node_token) = single_node(node);
        nodes_diff_var.push(diff_vars);
        tokens.extend(quote! {
            #node_token,
        });
    }
    (nodes_diff_var, tokens)
}

fn node_attributes(attributes: Vec<NodeAttribute>) -> (Vec<Expr>,TokenStream) {
    let mut tokens = TokenStream::new();
    let mut diff_vars = vec![];
    for attr in attributes {
        let (diff_var, attr_token) = attribute_to_tokens(attr);
        if let Some(diff_var) = diff_var{
            diff_vars.push(diff_var);
        }
        tokens.extend(quote! {
            #attr_token,
        });
    }
    (diff_vars, tokens)
}

fn attribute_to_tokens(attribute: NodeAttribute) -> (Option<Expr>, TokenStream) {
    match attribute {
        NodeAttribute::Block(block) => {
            (None,quote! {
                #[allow(unused_braces)]
                #block
            })
        }
        NodeAttribute::Attribute(attribute) => {
            let attr = attribute.key.to_string();
            let value = attribute.possible_value;
            match value {
                KeyedAttributeValue::Binding(binding) => {
                    (None,quote! {
                        compile_error!("Function binding is not supported! {:?}",#binding)
                    })
                }
                KeyedAttributeValue::Value(value) => {
                    let value = value.value;
                    let is_event = attr.starts_with("on_");
                    if is_event {
                        let event = quote::format_ident!("{attr}");
                        (None, quote! {
                            #[allow(unused_braces)]
                            sauron::html::events::#event(#value)
                        })
                    } else {
                        match value{
                            Expr::Lit(lit) => {
                                (None, quote! {
                                    #[allow(unused_braces)]
                                    sauron::html::attributes::attr(#attr, #lit)
                                })
                            }
                            _ => {
                                (Some(value.clone()), quote! {
                                    #[allow(unused_braces)]
                                    sauron::html::attributes::attr(#attr, #value)
                                })
                            }
                        }
                    }
                }
                KeyedAttributeValue::None => {
                    (None, quote! {
                        sauron::html::attributes::empty_attr()
                    })
                }
            }
        }
    }
}

fn braced_for_loop(block: &syn::Block) -> Option<&ExprForLoop> {
    let len = block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &block.stmts[0];
        match stmt {
            Stmt::Expr(Expr::ForLoop(expr), _semi) => Some(expr),
            _ => None,
        }
    }
}

fn lit_expr(block: &syn::Block) -> Option<&ExprLit> {
    let len = block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &block.stmts[0];
        match stmt {
            Stmt::Expr(Expr::Lit(lit), _semi) => Some(lit),
            _ => None,
        }
    }
}

fn some_expr(block: &syn::Block) -> Option<&Expr> {
    let len = block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &block.stmts[0];
        match stmt {
            Stmt::Expr(expr, _semi) => Some(expr),
            _ => None,
        }
    }
}
