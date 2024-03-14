use crate::dom::program::MountProcedure;
use crate::dom::{Application, Modifier, Program};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// a trait for implementing WebComponent in the DOM with custom tag
pub trait WebComponent {

    /// 
    type MSG;
    /// returns the attributes that is observed by this component
    /// These are the names of the attributes the component is interested in
    fn observed_attributes() -> Vec<&'static str>;

    /// This will be invoked when a component is used as a custom element
    /// and the attributes of the custom-element has been modified
    ///
    /// if the listed attributes in the observed attributes are modified
    fn attribute_changed(
        program: Program<Self>,
        attr_name: &str,
        old_value: Option<String>,
        new_value: Option<String>,
    ) where
        Self: Sized + Application;

    /// the component is attached to the dom
    fn connected_callback(&mut self);
    /// the component is removed from the DOM
    fn disconnected_callback(&mut self);

    /// the component is moved or attached to the dom
    fn adopted_callback(&mut self);
}

thread_local!(static REGISTER_CUSTOM_ELEMENT_FUNCTION: js_sys::Function = declare_custom_element_function());

/// register using custom element define
/// # Example:
/// ```rust,ignore
///  sauron::register_web_component("date-time", "DateTimeWidgetCustomElement");
/// ```
pub fn register_web_component(custom_tag: &str, adapter: JsValue, observed_attributes: JsValue) {
    log::info!("registering a custom element: {:?}", custom_tag);
    REGISTER_CUSTOM_ELEMENT_FUNCTION.with(|func| {
        func.call3(
            &JsValue::NULL,
            &JsValue::from_str(custom_tag),
            &adapter,
            &observed_attributes,
        )
        .expect("must call");
    })
}

/// TODO: refer to https://github.com/gbj/custom-elements
/// for improvements
/// dynamically create the function which will register the custom tag
///
/// This is needed since there is no way to do `class extends HTMLElement` in rust code
fn declare_custom_element_function() -> js_sys::Function {
    js_sys::Function::new_with_args(
        "custom_tag, adapter, observed_attributes",
        r#"
          if (window.customElements.get(custom_tag) === undefined ){
             window.customElements.define(custom_tag,
                class extends HTMLElement{
                    constructor(){
                        super();
                        // the adapter execute the closure which attached the `new` function into `this`
                        // object.
                        adapter(this);
                        // we then call that newly attached `new` function to give us an instance
                        // of the CustomElement WebComponent
                        this.instance = this.new(this);
                    }
                    static get observedAttributes(){
                        return observed_attributes;
                    }
                    connectedCallback(){
                        this.instance.connectedCallback();
                    }
                    disconnectedCallback(){
                        this.instance.disconnectedCallback();
                    }
                    adoptedCallback(){
                        this.instance.adoptedCallback();
                    }
                    attributeChangedCallback(name, oldValue, newValue){
                        this.instance.attributeChangedCallback(name, oldValue, newValue);
                    }
                    appendChild(child){
                        this.instance.appendChild(child);
                    }
                }
             );
         }"#,
    )
}

/*
impl<COMP, MSG> Application<MSG> for COMP
where
    COMP: Container<MSG, ()> + WebComponent<MSG> + 'static,
    MSG: 'static,
{
    fn init(&mut self) -> Cmd<Self, MSG> {
        Cmd::from(<Self as Container<MSG, ()>>::init(self))
    }

    fn update(&mut self, msg: MSG) -> Cmd<Self, MSG> {
        let effects = <Self as Container<MSG, ()>>::update(self, msg);
        Cmd::from(effects)
    }

    fn view(&self) -> Node<MSG> {
        <Self as Container<MSG, ()>>::view(self, [])
    }

    fn stylesheet() -> Vec<String> {
        <Self as Container<MSG, ()>>::stylesheet()
    }

    fn style(&self) -> Vec<String> {
        <Self as Container<MSG, ()>>::style(self)
    }
}
*/

/// A self contain web component
/// This is needed to move some of the code from the #custom_element macro
/// This is also necessary, since #[wasm_bindgen] macro can not process impl types which uses
/// generics, we use generics here to simplify the code and do the type checks for us, rather than
/// in the code derived from the #[web_component] macro
pub struct WebComponentWrapper<APP>
    where APP: Application
{
    /// the underlying program running this web component
    pub program: Program<APP>,
    /// the mount node for the program
    pub mount_node: web_sys::Node,
}

impl<APP> WebComponentWrapper<APP>
where
    APP: Application + WebComponent + Default + 'static,
{
    /// create a new web component, with the node as the target element to be mounted into
    pub fn new(node: JsValue) -> Self {
        let mount_node: web_sys::Node = node.unchecked_into();
        let program = Program::new(APP::default());
        Self {
            program,
            mount_node,
        }
    }

    /// When the attribute of the component is changed, this method will be called
    pub fn attribute_changed(&self, attr_name: &str, old_value: JsValue, new_value: JsValue) {
        let old_value = old_value.as_string();
        let new_value = new_value.as_string();
        APP::attribute_changed(self.program.clone(), attr_name, old_value, new_value);
    }

    /// called when the web component is mounted
    pub fn connected_callback(&mut self) {
        self.program
            .mount(&self.mount_node, MountProcedure::append_to_shadow());
        let static_style = <APP as Application>::stylesheet().join("");
        self.program.inject_style_to_mount(&static_style);
        let dynamic_style =
            <APP as Application>::style(&self.program.app_context.app.borrow()).join("");
        self.program.inject_style_to_mount(&dynamic_style);
        self.program
            .app_context
            .app
            .borrow_mut()
            .connected_callback();
        self.program
            .update_dom(&Modifier::default())
            .expect("must update dom");
    }

    /// called when the web component is removed
    pub fn disconnected_callback(&mut self) {
        self.program
            .app_context
            .app
            .borrow_mut()
            .disconnected_callback();
    }

    /// called when web componented is moved into other parts of the document
    pub fn adopted_callback(&mut self) {
        self.program.app_context.app.borrow_mut().adopted_callback();
    }
}
