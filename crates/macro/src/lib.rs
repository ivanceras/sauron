#![deny(
    warnings,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]

//! node macro facilitates users of sauron to use html-like syntax
//! for building view of web app components
use quote::ToTokens;

mod custom_element;
mod extract_skip_diff;
mod extract_template;
mod jss;
mod node;

/// Quasi-quoting macro for building sauron [Node]s.
///
/// The macro allows for specifying html-like elements and attributes, and
/// supports advanced interpolation and looping.
///
/// [Node]: https://docs.rs/sauron/0/sauron/type.Node.html
///
/// # Elements
///
/// Both open elements with a closing tag, and elements which are immediately
/// closed are supported:
///
/// ```rust
/// use sauron::{node,Node};
///
/// let _: Node<()> = node!(<input type="button" />);
/// let _: Node<()> = node!(<h1>"A title"</h1>);
/// ```
///
/// # Attributes
///
/// Attributes must be valid Rust identifiers. These are translated to
/// `kebab-case` and trimmed. So `x_data_` would be translated to `x-data`.
///
/// Any sort of literal (like `true` or `42u32`) is supported as an attribute
/// argument.
///
/// ```rust
/// use sauron::{node,Node};
///
/// let _: Node<()> = node!(<input x_data_="my data" />);
/// let _: Node<()> = node!(<input x_data_int_=42u32 x_data_bool_=true />);
/// ```
///
/// Attribute values can be interpolated. These expressions must produce
/// an attribute that can be converted into a [Value].
///
/// ```rust
/// use sauron::{node,Node};
///
/// struct Model {
///     value: String,
/// }
///
/// impl Model {
///     pub fn view(&self) -> Node<()> {
///         node!(<input value={self.value.clone()} />)
///     }
/// }
/// ```
///
/// Whole attributes can also be generated. These expressions must produce an
/// [Attribute].
///
/// ```rust
/// use sauron::{node,Node,html::attributes::classes_flag};
///
/// struct Model {
///     editing: bool,
///     completed: bool,
/// }
///
/// impl Model {
///     pub fn view(&self) -> Node<()> {
///         node!(<input {{classes_flag([
///             ("todo", true),
///             ("editing", self.editing),
///             ("completed", self.completed),
///         ])}} />)
///     }
/// }
/// ```
///
/// Finally, we also support empty attributes.
///
/// ```rust
/// use sauron::{node,Node};
///
/// let _: Node<()> = node!(<button disabled />);
/// ```
///
/// [Value]: https://docs.rs/sauron/0/sauron/html/attributes/enum.Value.html
/// [Attribute]: https://docs.rs/sauron/0/sauron/type.Attribute.html
///
/// # Event handlers
///
/// Event handlers are special attributes. Any attribute that starts with `on_`
/// will be matched with event handlers available in [sauron::dom::events].
///
/// ```rust
/// use sauron::{node,Node,
///     events::{InputEvent,KeyboardEvent},
/// };
///
/// enum Msg {
///     Update(String),
///     Add,
///     Nope,
/// }
///
/// struct Model {
///     value: String,
/// }
///
/// impl Model {
///    fn view(&self) -> Node<Msg> {
///        node! {
///            <input
///                class="new-todo"
///                id="new-todo"
///                placeholder="What needs to be done?"
///                value={self.value.to_string()}
///                on_input={|v: InputEvent| Msg::Update(v.value())}
///                on_keypress={|event: KeyboardEvent| {
///                    if event.key() == "Enter" {
///                        Msg::Add
///                    } else {
///                        Msg::Nope
///                    }
///                }} />
///         }
///     }
/// }
/// ```
///
/// [sauron::dom::events]: https://docs.rs/sauron/0/sauron/dom/events/index.html
///
/// # Loops
///
/// Loops are supported through a special construct. They look and behave like
/// regular Rust loops, except that whatever the loop body evaluates to will be
/// appended to the child of a node.
///
/// The loop body must evaluate to a [Node].
///
/// ```rust
/// use sauron::{node,Node,html::text};
///
/// struct Model {
///     items: Vec<String>,
/// }
///
/// impl Model {
///     pub fn view(&self) -> Node<()> {
///         node! {
///             <ul>
///                 {for item in self.items.iter() {
///                     text(item)
///                 }}
///             </ul>
///         }
///     }
/// }
/// ```
///
/// [Node]: https://docs.rs/sauron/0/sauron/type.Node.html
///
/// Note: `node!` macro is used since it is not an html tag
/// while most other framework uses `html!` macro, this prevents
/// the library to have collision with the `html` tag, when used as tag macro
#[proc_macro]
pub fn node(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    node::to_token_stream(input).into()
}

/// Generate a skip diff list based on the node used in the view
/// ```rust
/// use sauron_macro::extract_skip_diff;
///
/// let skip = extract_skip_diff!{<ul class="some-list"></ul>};
/// assert_eq!(skip, sauron::skip_if(true,[]), "skip if all attribute values are static");
/// ```
///
/// ```rust
/// use sauron_macro::extract_skip_diff;
///
/// let skip = extract_skip_diff!{"item 1"};
/// assert_eq!(skip, sauron::skip_if(true,[]), "skip raw text");
/// ```
///
#[proc_macro]
pub fn extract_skip_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    extract_skip_diff::to_token_stream(input).into()
}

/// extract template by including only attributes and elements that is static str
/// ```rust
/// use sauron_macro::extract_template;
/// use sauron::*;
///
/// let template: Node<()> = extract_template!{<ul class="some-list"></ul>};
/// assert_eq!(template, node!{<ul class="some-list"></ul>}, "same template as the view");
///
/// let template: Node<()> = extract_template!{<h1>Hello world!</h1>};
/// assert_eq!(template, node!{<h1>Hello world!</h1>}, "raw texts");
///
/// let name = "bob";
/// let template: Node<()> = extract_template!{<h1>{text(format!("Hello {}!", name))}</h1>};
/// assert_eq!(template, node!{<h1>PLACEHOLDER</h1>}, "formated text");
/// ```
#[proc_macro]
pub fn extract_template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    extract_template::to_token_stream(input).into()
}

/// derive code for a certain CustomElement implementation to have the necessary types and glue
/// code which register itself into the the DOM customElementRegistry.
/// This also adds the register function into the module it is called in, so make sure you only
/// use this in each module, multiple web_components should be in different modules
#[proc_macro_attribute]
pub fn custom_element(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    custom_element::to_token_stream(attr, input)
}

/// build a css string
///
/// # Example:
/// ```rust
/// use sauron::jss;
///
/// let css = jss!(
///     ".layer": {
///         background_color: "red",
///         border: "1px solid green",
///     },
///
///     ".hide .layer": {
///         opacity: 0,
///     },
/// );
///
/// let expected = "\
///     .layer {\
///     \n  background-color: red;\
///     \n  border: 1px solid green;\
///     \n}\
///     \n\
///     \n.hide .layer {\
///     \n  opacity: 0;\
///     \n}\
///     \n";
/// assert_eq!(expected, css);
/// ```
#[proc_macro]
pub fn jss(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let style_sheet = syn::parse_macro_input!(input as jss::StyleSheet);
    style_sheet.to_token_stream().into()
}

/// build css string that has media selector or any other conditional group
///
/// # Example:
/// ```rust
/// use sauron::jss_with_media;
///
/// let css = jss_with_media!(
///     "@media screen and (max-width: 800px)": {
///       ".layer": {
///         width: "100%",
///       }
///     },
/// );
///
/// let expected = "\
///     @media screen and (max-width: 800px) {\
///         \n.layer {\
///         \n  width: 100%;\
///         \n}\
///         \n\
///         \n}\
///         \n";
/// assert_eq!(expected, css);
/// ```
#[proc_macro]
pub fn jss_with_media(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let css_media = syn::parse_macro_input!(input as jss::StyleSheetWithConditionalGroup);
    css_media.to_token_stream().into()
}

/// build a style attribute
///
/// # Example:
/// ```rust
/// use sauron::style;
/// use sauron::html::units::{px, percent};
/// use sauron::html::attributes::{Attribute,attr};
///
/// let s1: Attribute<()> = style! {
///     background_color: "red",
///     border: (px(1), "solid", "green"),
///     width: percent(100),
/// };
/// let expected: Attribute<()> = attr("style","background-color:red;border:1px solid green;width:100%;");
/// assert_eq!(expected.render_to_string(), s1.render_to_string());
/// ```
#[proc_macro]
pub fn style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let style = syn::parse_macro_input!(input as jss::Style);
    style.to_attr_tokens().into()
}
