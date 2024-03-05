use syn::Result;

#[derive(Debug)]
struct CustomElementParts {
    struct_name: String,
    custom_tag: String,
    msg_type: syn::Ident,
    self_type: syn::Ident,
    self_type_has_empty_tuple_generics: bool,
}

impl CustomElementParts {
    fn parse(attr_parsed: &syn::LitStr, item_impl: &syn::ItemImpl) -> Result<Self> {
        let custom_tag = attr_parsed.value();
        assert!(item_impl.trait_.is_some(), "must be a trait");
        let (_not, path, _for) = item_impl.trait_.as_ref().expect("must have a trait");
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
        let msg_type = msg_types[0].clone();
        let self_types = extract_idents_from_type_path(&item_impl.self_ty);
        let self_type_has_empty_tuple_generics = contains_empty_tuple(&path);
        assert_eq!(self_types.len(), 1);
        let self_type = self_types[0].clone();
        let struct_name = self_types
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<_>>()
            .join("");

        Ok(Self {
            struct_name,
            custom_tag,
            msg_type,
            self_type,
            self_type_has_empty_tuple_generics,
        })
    }

    fn custom_element_name(&self) -> String {
        format!("{}__CustomElement", self.struct_name)
    }

    fn widget_wrapper(&self) -> syn::Ident {
        quote::format_ident!("{}", self.custom_element_name())
    }

    fn simplified_type(&self) -> syn::Ident {
        quote::format_ident!("{}__Simple", self.struct_name)
    }

    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let simplified_type = self.simplified_type();
        let self_type = &self.self_type;
        let widget_wrapper = self.widget_wrapper();
        let custom_element_name = self.custom_element_name();
        let orig_msg_type = &self.msg_type;
        let msg_type = if self.self_type_has_empty_tuple_generics {
            quote::quote! {
               #orig_msg_type<()>
            }
        } else {
            quote::quote! {
                #orig_msg_type
            }
        };
        let custom_tag = &self.custom_tag;

        quote::quote! {
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
        }
    }
}

pub fn to_token_stream(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut tokens = proc_macro::TokenStream::new();

    let input_clone = input.clone();
    let attr_parsed = syn::parse_macro_input!(attr as syn::LitStr);
    let item_impl = syn::parse_macro_input!(input_clone as syn::ItemImpl);

    let custom_element = CustomElementParts::parse(&attr_parsed, &item_impl)
        .expect("must parse custom elements parts");

    let expanded = custom_element.to_tokens();

    tokens.extend(proc_macro::TokenStream::from(expanded));
    tokens.extend(input);
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

fn contains_empty_tuple(path: &syn::Path) -> bool {
    path.segments
        .last()
        .map(|segment| is_generic_empty_tuple(&segment.arguments))
        .unwrap_or(false)
}

/// return true if this path arguments is `<()>`
fn is_generic_empty_tuple(arguments: &syn::PathArguments) -> bool {
    if let syn::PathArguments::AngleBracketed(abga) = arguments {
        if let Some(syn::GenericArgument::Type(syn::Type::Path(type_path))) = abga.args.first() {
            if let Some(last_segment) = type_path.path.segments.last() {
                if let syn::PathArguments::AngleBracketed(abga) = &last_segment.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Tuple(type_tuple))) =
                        abga.args.first()
                    {
                        return type_tuple.elems.is_empty();
                    }
                }
            }
        }
    }
    false
}
