use proc_macro2::TokenStream;
use quote::quote;
use rstml::node::{Node, NodeAttribute, NodeBlock};
use syn::{Expr, ExprForLoop, ExprIf, Stmt};

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    match rstml::parse(input) {
        Ok(nodes) => do_extract(&nodes),
        Err(error) => error.to_compile_error(),
    }
}

fn do_extract(nodes: &[Node]) -> TokenStream{
    let skip_tree = from_multiple_nodes(&nodes);
    quote! {
        #skip_tree.collapse_children()
    }
}

fn from_multiple_nodes(nodes: &[Node]) -> TokenStream {
    let only_one_node = nodes.len() == 1;
    if only_one_node {
        let node_tokens = from_single_node(&nodes[0]);
        quote! {
            #node_tokens
        }
    } else {
        let children_tokens = nodes_to_tokens(&nodes);
        quote! {
            sauron::skip_if(false,  [#children_tokens])
        }
    }
}

fn from_single_node(node: &Node) -> TokenStream {
    match node {
        Node::Element(elm) => {
            //also returns true if there is no attributes
            let is_attrs_literal = is_all_literal_attributes(&elm.open_tag.attributes);
            let children = nodes_to_tokens(&elm.children);
            quote! {::sauron::skip_if(#is_attrs_literal, [#children])}
        }
        Node::Fragment(fragment) => from_multiple_nodes(&fragment.children),
        Node::Text(_) | Node::RawText(_) | Node::Comment(_) | Node::Doctype(_) => {
            quote! {::sauron::skip_if(true, [])}
        }
        Node::Block(block) => match block {
            NodeBlock::Invalid { .. } => {
                quote! {
                    compile_error!("invalid block: {:?}", #block);
                }
            }
            NodeBlock::ValidBlock(block) => {
                if let Some(ExprForLoop { .. }) = braced_for_loop(&block) {
                    quote! {skip_if(false, [])}
                } else if let Some(ExprIf { .. }) = braced_if_expr(&block) {
                    quote! {skip_if(false, [])}
                } else {
                    quote! {skip_if(false, [])}
                }
            }
        },
    }
}

fn nodes_to_tokens(nodes: &[Node]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for node in nodes {
        let node_token = from_single_node(node);
        tokens.extend(quote! {
            #node_token,
        });
    }

    tokens
}

fn is_all_literal_attributes(attributes: &[NodeAttribute]) -> bool {
    attributes.iter().all(is_literal_attribute)
}

pub(crate) fn is_literal_attribute(attribute: &NodeAttribute) -> bool {
    match attribute {
        NodeAttribute::Block(_block) => false,
        NodeAttribute::Attribute(attribute) => attribute.value_literal_string().is_some(),
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

fn braced_if_expr(block: &syn::Block) -> Option<&ExprIf> {
    let len = block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &block.stmts[0];
        match stmt {
            Stmt::Expr(Expr::If(expr), _semi) => Some(expr),
            _ => None,
        }
    }
}
