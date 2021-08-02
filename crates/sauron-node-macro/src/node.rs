use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, ExprBlock, ExprForLoop, Ident, Stmt};
use syn_rsx::{Node, NodeType, ParserConfig};

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    match syn_rsx::parse_with_config(
        input,
        ParserConfig::new()
            .number_of_top_level_nodes(1)
            .type_of_top_level_nodes(NodeType::Element),
    ) {
        Ok(mut nodes) => node_to_tokens(
            nodes.pop().expect("unable to convert node to tokens"),
        ),
        Err(error) => error.to_compile_error(),
    }
}

fn node_to_tokens(node: Node) -> TokenStream {
    let mut tokens = TokenStream::new();

    // NodeType::Element nodes can't have no name
    let name = node.name_as_string().expect("node should have a name");

    let attributes = node
        .attributes
        .iter()
        .map(|attribute| attribute_to_tokens(attribute))
        .collect::<Vec<_>>();

    let children_tokens = children_to_tokens(node.children);

    tokens.extend(quote! {{
        #[allow(unused_braces)]
        {
            #children_tokens
            if let Some(ns) = sauron::parser::tag_namespace(#name){
                sauron::html::html_element_ns(#name, ns, vec![#(#attributes),*], children)
            }else{
                sauron::html::html_element(#name, vec![#(#attributes),*], children)
            }
        }
    }});

    tokens
}

fn attribute_to_tokens(attribute: &Node) -> TokenStream {
    match &attribute.value {
        Some(value) => {
            match attribute.node_type {
                NodeType::Block => {
                    quote! {
                        sauron::Attribute::from(#value)
                    }
                }
                NodeType::Attribute => {
                    // NodeType::Attribute nodes can't have no name
                    let name = attribute
                        .name_as_string()
                        .expect("attribute should have name");

                    if name.starts_with("on_") {
                        let name = quote::format_ident!("{}", name);
                        quote::quote! {
                            sauron::events::#name(#value)
                        }
                    } else {
                        let name = convert_name(&name);
                        quote::quote! {
                            sauron::Attribute::new(
                                None,
                                #name,
                                sauron::html::attributes::AttributeValue::Simple(
                                    sauron::html::attributes::Value::from(#value)
                                )
                            )
                        }
                    }
                }
                _ => {
                    quote! {
                        compile_error!("Unexpected NodeType")
                    }
                }
            }
        }
        None => {
            let name = convert_name(
                &attribute
                    .name_as_string()
                    .expect("attribute should have a name"),
            );
            quote! {
                sauron::Attribute::new(
                    None,
                    #name,
                    sauron::html::attributes::AttributeValue::Empty,
                )
            }
        }
    }
}

fn children_to_tokens(children: Vec<Node>) -> TokenStream {
    let receiver = Ident::new("children", Span::call_site());
    let mut tokens = TokenStream::new();
    if !children.is_empty() {
        let count = children.len();

        tokens.extend(quote! {
            let mut #receiver = Vec::with_capacity(#count);
        });

        for child in children {
            match child.node_type {
                NodeType::Element => {
                    let node = node_to_tokens(child);
                    tokens.extend(quote! {
                        #receiver.push(#node);
                    });
                }
                NodeType::Text => {
                    let s = child
                        .value_as_string()
                        .expect("expecting a string on a text node");
                    tokens.extend(quote! {
                        #receiver.push(sauron::html::text(#s));
                    });
                }
                NodeType::Block => match child.value {
                    Some(syn::Expr::Block(expr)) => {
                        match braced_for_loop(&expr) {
                            Some(ExprForLoop {
                                pat, expr, body, ..
                            }) => {
                                tokens.extend(quote! {
                                        for #pat in #expr {
                                            #receiver.push(sauron::Node::from(#body));
                                        }
                                    });
                            }
                            _ => {
                                tokens.extend(quote! {
                                    #receiver.push(sauron::Node::from(#expr));
                                });
                            }
                        }
                    }
                    _ => {
                        return quote! {
                            compile_error!("Unexpected missing block for NodeType::Block")
                        }
                    }
                },
                _ => {
                    return quote! {
                        compile_error!(format!("Unexpected NodeType for child: {}", child.node_type))
                    }
                }
            }
        }
    } else {
        tokens.extend(quote! {
            let #receiver = Vec::new();
        });
    }

    tokens
}

fn braced_for_loop<'a>(expr: &'a ExprBlock) -> Option<&'a ExprForLoop> {
    let len = expr.block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &expr.block.stmts[0];
        match stmt {
            Stmt::Expr(expr) => match expr {
                Expr::ForLoop(expr) => Some(expr),
                _ => None,
            },
            _ => None,
        }
    }
}

fn convert_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());

    for c in name.trim_matches('_').chars() {
        match c {
            '_' => out.push('-'),
            c => out.push(c),
        }
    }

    out
}
