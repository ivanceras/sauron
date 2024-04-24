//! the view provides view method of Application or Components
//! additionally, template and skip_diff is also extracted from the same view function
//!
use proc_macro2::TokenStream;
use quote::quote;

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    let view = crate::node::to_token_stream(input.clone());
    let skip_diff = crate::extract_skip_diff::to_token_stream(input.clone());
    quote! {
        fn view(&self) -> Node<Self::MSG> {
             ::sauron::Node::Leaf(sauron::vdom::Leaf::TemplatedView(sauron::vdom::TemplatedView{
                        view: Box::new(#view),
                        skip_diff: std::rc::Rc::new(||#skip_diff),
                    }))
        }
    }
}
