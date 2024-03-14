//! the view provides view method of Application or Components
//! additionally, template and skip_diff is also extracted from the same view function
//!
use proc_macro2::TokenStream;
use quote::quote;

pub fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    let view_node = crate::node::to_token_stream(input.clone());
    let skip_diff = crate::extract_skip_diff::to_token_stream(input.clone());
    let template = crate::extract_template::to_token_stream(input);
    quote!{
        fn view(&self) -> Node<Self::MSG> {
            #view_node
        }

        fn skip_diff(&self) -> Option<SkipDiff> {
            Some(#skip_diff)
        }

        fn template(&self) -> Option<Node<Self::MSG>>{
            Some(#template)
        }
    }
}
