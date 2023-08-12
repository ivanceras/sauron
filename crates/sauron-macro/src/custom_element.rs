pub fn to_token_stream(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut tokens = proc_macro::TokenStream::new();
    let attr_parsed = syn::parse_macro_input!(attr as syn::LitStr);
    let input_clone = input.clone();
    let result = syn::parse_macro_input!(input_clone as syn::ItemImpl);
    assert!(result.trait_.is_some(), "must be a trait");
    let (_not, path, _for) = &result.trait_.expect("must have a trait");
    let custom_element_trait = extract_idents_from_path(path);
    let custom_element_last = custom_element_trait
        .last()
        .expect("must have a custom element trait");
    assert_eq!(
        "WebComponent",
        custom_element_last.to_string(),
        "must only be used on impl WebComponent"
    );
    let msg_types = extract_custom_element_msg(path);
    assert_eq!(msg_types.len(), 1);
    let msg_type = msg_types[0];
    let self_types = extract_idents_from_type_path(&result.self_ty);
    assert_eq!(self_types.len(), 1);
    let self_type = self_types[0];
    let struct_name = self_types
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>()
        .join("");

    let custom_element_name = format!("{struct_name}__CustomElement");
    let widget_wrapper = quote::format_ident!("{custom_element_name}");
    let simplified_type = quote::format_ident!("{struct_name}__Simple");
    let custom_tag = attr_parsed.value();

    let expanded = quote::quote! {
        type #simplified_type = #self_type<()>;
        #[wasm_bindgen]
        pub struct #widget_wrapper{
            web_component: sauron::dom::WebComponentWrapper<#simplified_type, #msg_type>,
        }

        #[wasm_bindgen]
        impl #widget_wrapper{
            #[wasm_bindgen(constructor)]
            pub fn new(node: JsValue) -> Self {
                Self{
                    web_component: sauron::dom::WebComponentWrapper::new(node)
                }
            }

            #[allow(unused)]
            #[wasm_bindgen(getter, static_method_of = Self, js_name = observedAttributes)]
            pub fn observed_attributes() -> JsValue {
                let attributes = #simplified_type::observed_attributes();
                serde_wasm_bindgen::to_value(&attributes).expect("convert to value")
            }

            #[wasm_bindgen(method, js_name = attributeChangedCallback)]
            pub fn attribute_changed_callback(
                &self,
                attr_name: &str,
                old_value: JsValue,
                new_value: JsValue,
            ) {
                self.web_component.attribute_changed(attr_name, old_value, new_value);
            }

            #[wasm_bindgen(method, js_name = connectedCallback)]
            pub fn connected_callback(&mut self) {
                self.web_component.connected_callback()
            }

            #[wasm_bindgen(method, js_name = disconnectedCallback)]
            pub fn disconnected_callback(&mut self) {
                self.web_component.disconnected_callback()
            }

            #[wasm_bindgen(method, js_name = adoptedCallback)]
            pub fn adopted_callback(&mut self) {
                self.web_component.adopted_callback()
            }

            fn struct_name() -> &'static str {
               #custom_element_name
            }

            pub fn register() {
                let constructor: Closure<dyn FnMut(JsValue)> = Closure::new(|node: JsValue| {
                    let new:Closure<dyn FnMut(JsValue) -> Self> = Closure::new(|node: JsValue| {
                        Self::new(node)
                    });
                    // assign the `new` closure into the `new` function to be called in the
                    // javascript side.
                    js_sys::Reflect::set(&node, &JsValue::from_str("new"), &new.into_js_value())
                        .unwrap_throw();
                });

                sauron::dom::register_web_component(
                    #custom_tag,
                    constructor.into_js_value(),
                    Self::observed_attributes(),
                );
            }
        }

        pub fn register(){
            #widget_wrapper::register()
        }
    };

    tokens.extend(input);
    tokens.extend(proc_macro::TokenStream::from(expanded));
    tokens
}

fn extract_idents_from_path(path: &syn::Path) -> Vec<&proc_macro2::Ident> {
    path.segments
        .iter()
        .map(|segment| &segment.ident)
        .collect::<Vec<_>>()
}

fn extract_idents_from_type_path(type_: &syn::Type) -> Vec<&proc_macro2::Ident> {
    if let syn::Type::Path(type_path) = type_ {
        let generic_ident = extract_idents_from_path(&type_path.path);
        return generic_ident;
    }
    vec![]
}

fn extract_custom_element_msg(path: &syn::Path) -> Vec<&proc_macro2::Ident> {
    if let Some(last_path) = path.segments.last() {
        if let syn::PathArguments::AngleBracketed(abga) = &last_path.arguments {
            if let Some(syn::GenericArgument::Type(type_)) = abga.args.first() {
                return extract_idents_from_type_path(type_);
            }
        }
    }
    vec![]
}
