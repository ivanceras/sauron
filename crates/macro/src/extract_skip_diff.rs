use proc_macro2::TokenStream;
use quote::quote;
use rstml::node::{Node, NodeAttribute, NodeBlock};

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    match rstml::parse(input) {
        Ok(nodes) => do_extract(&nodes),
        Err(error) => error.to_compile_error(),
    }
}

fn do_extract(nodes: &[Node]) -> TokenStream {
    let skip_tree = from_multiple_nodes(nodes);
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
        let children_tokens = nodes_to_tokens(nodes);
        quote! {
            sauron::skip_if(false,  [#children_tokens])
        }
    }
}

fn from_single_node(node: &Node) -> TokenStream {
    match node {
        Node::Element(elm) => {
            //also returns true if there is no attributes
            let skip_attrs = if is_all_literal_attributes(&elm.open_tag.attributes) {
                quote! {
                   ::sauron::dom::skip_diff::SkipAttrs::All
                }
            } else {
                let indices = literal_attributes_indices(&elm.open_tag.attributes);
                let mut indices_tokens = TokenStream::new();
                for index in indices {
                    indices_tokens.extend(quote! {
                        #index,
                    });
                }
                quote! {
                    ::sauron::dom::skip_diff::SkipAttrs::Indices(vec![#indices_tokens])
                }
            };
            let children = nodes_to_tokens(&elm.children);
            quote! {
                ::sauron::SkipDiff{
                    skip_attrs: #skip_attrs,
                    children: vec![#children],
                }
            }
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
            NodeBlock::ValidBlock(_block) => {
                quote! {
                    sauron::SkipDiff::block()
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

fn literal_attributes_indices(attributes: &[NodeAttribute]) -> Vec<usize> {
    attributes
        .iter()
        .enumerate()
        .filter_map(|(i, att)| {
            if is_literal_attribute(att) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

pub(crate) fn is_literal_attribute(attribute: &NodeAttribute) -> bool {
    match attribute {
        NodeAttribute::Block(_block) => false,
        NodeAttribute::Attribute(attribute) => attribute.value_literal_string().is_some(),
    }
}
