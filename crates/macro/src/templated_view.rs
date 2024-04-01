//!
use proc_macro2::TokenStream;
use quote::quote;

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    let view = crate::node::to_token_stream(input.clone());
    let skip_diff = crate::extract_skip_diff::to_token_stream(input.clone());
    let template = crate::extract_template::to_token_stream(input);
    quote! {
         ::sauron::Node::Leaf(sauron::vdom::Leaf::TemplatedView(sauron::vdom::TemplatedView{
                    view: Box::new(#view),
                    template: std::rc::Rc::new(||#template),
                    skip_diff: std::rc::Rc::new(||#skip_diff), 
                }))
    }
}
