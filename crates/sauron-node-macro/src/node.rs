use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, ExprBlock, ExprForLoop, Ident, Stmt};
use syn_rsx::{Node, NodeType, ParserConfig};

pub(crate) mod lookup;

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    match syn_rsx::parse_with_config(input, ParserConfig::new()) {
        Ok(mut nodes) => {
            if nodes.len() == 1 {
                let node =
                    nodes.pop().expect("unable to convert node to tokens");
                node_to_tokens(node)
            } else {
                node_list_to_tokens(nodes)
            }
        }
        Err(error) => error.to_compile_error(),
    }
}

fn node_list_to_tokens(nodes: Vec<Node>) -> TokenStream {
    let mut tokens = TokenStream::new();
    let children_tokens = children_to_tokens(nodes);
    tokens.extend(quote! {{
        #children_tokens
        sauron::html::node_list(children)
    }});
    tokens
}

fn node_to_tokens(node: Node) -> TokenStream {
    let mut tokens = TokenStream::new();

    // NodeType::Element nodes can't have no name
    let name = node.name_as_string().expect("node should have a name");

    let attributes = node.attributes.iter().map(attribute_to_tokens);

    let children_tokens = children_to_tokens(node.children);

    let self_closing = lookup::is_self_closing(&name);
    let namespace = lookup::tag_namespace(&name);

    if let Some(namespace) = namespace {
        tokens.extend(
                quote! {{
                    #[allow(unused_braces)]
                    {
                        #children_tokens
                        sauron::html::element_ns(Some(#namespace), #name, [#(#attributes),*], children, #self_closing)
                    }
                }}
            );
    } else {
        tokens.extend(
                quote! {{
                    #[allow(unused_braces)]
                    {
                        #children_tokens
                        sauron::html::element_ns(None, #name, [#(#attributes),*], children, #self_closing)
                    }
                }}
            );
    }
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
                            sauron::html::attributes::attr(#name, #value)
                        }
                    }
                    /*
                    quote::quote! {
                        sauron::html::attributes::attr(#name, #value)
                    }
                    */
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
                NodeType::Comment => {
                    let s = child
                        .value_as_string()
                        .expect("expecting a string on a comment node");
                    tokens.extend(quote! {
                        #receiver.push(sauron::html::comment(#s));
                    });
                }
                NodeType::Doctype => {
                    let value = child
                        .value_as_string()
                        .expect("expecting a string value on a doctype");
                    tokens.extend(quote! {
                        #receiver.push(sauron::html::doctype(#value));
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

fn braced_for_loop(expr: &ExprBlock) -> Option<&ExprForLoop> {
    let len = expr.block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &expr.block.stmts[0];
        match stmt {
            Stmt::Expr(Expr::ForLoop(expr)) => Some(expr),
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
