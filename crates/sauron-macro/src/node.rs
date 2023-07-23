use proc_macro2::TokenStream;
use quote::quote;
use rstml::node::{KeyedAttributeValue, Node, NodeAttribute, NodeBlock};
use syn::{Expr, ExprForLoop, Stmt};

pub(crate) mod lookup;

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    match rstml::parse(input) {
        Ok(nodes) => multiple_nodes(nodes),
        Err(error) => error.to_compile_error(),
    }
}

fn multiple_nodes(mut nodes: Vec<Node>) -> TokenStream {
    let only_one_node = nodes.len() == 1;
    if only_one_node {
        let node_tokens = single_node(nodes.remove(0));
        quote! {
            #node_tokens
        }
    } else {
        let children_tokens = nodes_to_tokens(nodes);
        quote! {
            sauron::html::node_list([
                #children_tokens
            ])
        }
    }
}

fn single_node(node: Node) -> TokenStream {
    match node {
        Node::Element(elm) => {
            let open_tag = elm.open_tag;
            let tag = open_tag.name.to_string();

            let self_closing = lookup::is_self_closing(&tag);
            let namespace = lookup::tag_namespace(&tag);
            let attributes = node_attributes(open_tag.attributes);
            let children = nodes_to_tokens(elm.children);
            if let Some(namespace) = namespace {
                quote! {
                    sauron::html::element_ns(Some(#namespace), #tag, [#attributes], [#children], #self_closing)
                }
            } else {
                quote! {
                    sauron::html::element_ns(None, #tag, [#attributes], [#children], #self_closing)
                }
            }
        }
        Node::Fragment(fragment) => multiple_nodes(fragment.children),
        Node::Text(node_text) => {
            let text = node_text.value_string();
            quote! {
                sauron::html::text(#text)
            }
        }
        Node::RawText(raw_text) => {
            let text = raw_text.to_token_stream_string();
            quote! {
                sauron::html::text(#text)
            }
        }
        Node::Comment(comment) => {
            let comment_text = comment.value.value();
            quote! {
                sauron::html::comment(#comment_text)
            }
        }
        Node::Doctype(doctype) => {
            let value = doctype.value.to_token_stream_string();
            quote! {
                sauron::html::doctype(#value)
            }
        }
        Node::Block(block) => match block {
            NodeBlock::Invalid { .. } => {
                quote! {
                    compile_error!("invalid block: {:?}", #block);
                }
            }
            NodeBlock::ValidBlock(block) => match braced_for_loop(&block) {
                Some(ExprForLoop {
                    pat, expr, body, ..
                }) => {
                    quote! {
                        {
                            let mut receiver = vec![];
                            for #pat in #expr {
                                #[allow(unused_braces)]
                                receiver.push(#body)
                            }
                            sauron::html::node_list(receiver)
                        }
                    }
                }
                _ => {
                    quote! {
                        #block
                    }
                }
            },
        },
    }
}

fn nodes_to_tokens(nodes: Vec<Node>) -> TokenStream {
    let mut tokens = TokenStream::new();
    for node in nodes {
        let node_token = single_node(node);
        tokens.extend(quote! {
            #node_token,
        });
    }

    tokens
}

fn node_attributes(attributes: Vec<NodeAttribute>) -> TokenStream {
    let mut tokens = TokenStream::new();
    for attr in attributes {
        let attr_token = attribute_to_tokens(attr);
        tokens.extend(quote! {
            #attr_token,
        });
    }
    tokens
}

fn attribute_to_tokens(attribute: NodeAttribute) -> TokenStream {
    match attribute {
        NodeAttribute::Block(block) => {
            quote! {
                #[allow(unused_braces)]
                #block
            }
        }
        NodeAttribute::Attribute(attribute) => {
            let attr = attribute.key.to_string();
            let value = attribute.possible_value;
            match value {
                KeyedAttributeValue::Binding(binding) => {
                    quote! {
                        compile_error!("Function binding is not supported! {:?}",#binding)
                    }
                }
                KeyedAttributeValue::Value(value) => {
                    let value = value.value;
                    let is_event = attr.starts_with("on_");
                    if is_event {
                        let event = quote::format_ident!("{attr}");
                        quote! {
                            #[allow(unused_braces)]
                            sauron::html::events::#event(#value)
                        }
                    } else {
                        quote! {
                            #[allow(unused_braces)]
                            sauron::html::attributes::attr(#attr, #value)
                        }
                    }
                }
                KeyedAttributeValue::None => {
                    quote! {
                        sauron::html::attributes::empty_attr()
                    }
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
