use quote::quote;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn custom_element(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let custom_tag: proc_macro2::Literal =
        syn::parse(attr).expect("must be a literal");
    let impl_item: syn::ItemImpl = syn::parse(item)
        .expect("Expecting custom_element macro to be used in impl trait");

    let (_, path, _) = &impl_item
        .trait_
        .as_ref()
        .expect("must have a trait implementation");

    let component: &syn::PathSegment =
        &path.segments.last().expect("must have a last segment");

    let component_ident = component.ident.to_string();

    match &*component_ident {
        "Component" => impl_component(&impl_item, &custom_tag, component),
        "Container" => impl_component(&impl_item, &custom_tag, component),
        "Application" => panic!("Application trait is not supported"),
        _ => panic!("unsupported trait implementation: {}", component_ident),
    }
}

fn impl_simple_component(
    impl_item: &syn::ItemImpl,
    custom_tag: &proc_macro2::Literal,
    component: &proc_macro2::Ident,
    component_msg: &syn::PathSegment,
    derive_component: &proc_macro2::Ident,
    derive_msg: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let derive_component_str = derive_component.to_string();
    quote! {

        #impl_item

        pub const CUSTOM_TAG: &str = #custom_tag;
        pub const DERIVE_COMPONENT: &str = #derive_component_str;
        pub const SUPER_CLASS: &str = "HTMLElement";

        #[allow(non_camel_case_types)]
        pub struct #derive_msg(#component_msg);

        #[allow(non_camel_case_types)]
        #[wasm_bindgen]
        pub struct #derive_component{
            program: Program<#component<#derive_msg>, #derive_msg>,
        }

        #[wasm_bindgen]
        impl #derive_component {
            #[wasm_bindgen(constructor)]
            pub fn new(node: JsValue) -> Self {
                use sauron::wasm_bindgen::JsCast;
                log::info!("constructor..");
                let mount_node: &web_sys::Node = node.unchecked_ref();
                Self {
                    program: Program::new(
                        #component::default(),
                        mount_node,
                        false,
                        true,
                    ),
                }
            }

            #[wasm_bindgen(method)]
            pub fn observed_attributes() -> JsValue {
                JsValue::from_serde(&<#component::<#derive_msg> as Component<#component_msg, #derive_msg>>::observed_attributes())
                    .expect("must parse from serde")
            }

            #[wasm_bindgen(method)]
            pub fn attribute_changed_callback(&self) {
                use std::ops::DerefMut;
                use sauron::wasm_bindgen::JsCast;
                log::info!("attribute changed...");
                let mount_node = self.program.mount_node();
                let mount_element: &web_sys::Element = mount_node.unchecked_ref();
                let attribute_names = mount_element.get_attribute_names();
                let len = attribute_names.length();
                let mut attribute_values: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
                for i in 0..len {
                    let name = attribute_names.get(i);
                    let attr_name =
                        name.as_string().expect("must be a string attribute");
                    if let Some(attr_value) = mount_element.get_attribute(&attr_name) {
                        attribute_values.insert(attr_name, attr_value);
                    }
                }
                <#component<#derive_msg> as Component<#component_msg, #derive_msg>>::attributes_changed(self.program.app.borrow_mut().deref_mut(), attribute_values);
            }

            #[wasm_bindgen(method)]
            pub fn connected_callback(&mut self) {
                use std::ops::Deref;
                self.program.mount();
                log::info!("Component is connected..");
                let component_style = <#component<#derive_msg> as Component<#component_msg, #derive_msg>>::style(self.program.app.borrow().deref());
                self.program.inject_style_to_mount(&component_style);
                self.program.update_dom();
            }
            #[wasm_bindgen(method)]
            pub fn disconnected_callback(&mut self) {
                log::info!("Component is disconnected..");
            }
            #[wasm_bindgen(method)]
            pub fn adopted_callback(&mut self) {
                log::info!("Component is adopted..");
            }

        }

        impl Application<#derive_msg> for #component<#derive_msg> {
            fn update(&mut self, msg: #derive_msg) -> Cmd<Self, #derive_msg> {
                let mount_attributes = <Self as Component<#component_msg, #derive_msg>>::attributes_for_mount(self);
                Cmd::batch([
                    Cmd::from(
                        <Self as Component<#component_msg, #derive_msg>>::update(
                            self, msg.0,
                        )
                        .localize(#derive_msg),
                    ),
                    Cmd::new(|program| {
                        program.update_mount_attributes(mount_attributes);
                    }),
                ])
            }

            fn view(&self) -> Node<#derive_msg> {
                <Self as Component<#component_msg, #derive_msg>>::view(self)
                    .map_msg(#derive_msg)
            }
        }

        //#[wasm_bindgen]
        pub fn register(){
            sauron::register_custom_element(CUSTOM_TAG, DERIVE_COMPONENT, SUPER_CLASS);
        }

    }
}

/// Generate the component registration for component that has a Component trait implementation
fn impl_component(
    impl_item: &syn::ItemImpl,
    custom_tag: &proc_macro2::Literal,
    component: &syn::PathSegment,
) -> proc_macro::TokenStream {
    let self_type = &impl_item.self_ty;
    if let syn::Type::Path(type_path) = self_type.as_ref() {
        let (msg, component_msg) = extract_path_component_msg(&component);

        let component = &type_path.path.segments[0].ident;
        let derive_component = proc_macro2::Ident::new(
            &format!("_{}__CustomElement", component),
            proc_macro2::Span::call_site(),
        );
        let derive_msg = proc_macro2::Ident::new(
            &format!("_{}__CustomMsg", component),
            proc_macro2::Span::call_site(),
        );

        let tokens = impl_simple_component(
            impl_item,
            custom_tag,
            component,
            component_msg,
            &derive_component,
            &derive_msg,
        );
        tokens.into()
    } else {
        panic!("Expecting a Path");
    }
}

fn extract_path_component_msg(
    path_segment: &syn::PathSegment,
) -> (&proc_macro2::Ident, &syn::PathSegment) {
    if let syn::PathArguments::AngleBracketed(component_msg) =
        &path_segment.arguments
    {
        assert!(component_msg.args.len() > 0);
        let mut component_msg_iter = component_msg.args.iter();
        let first_arg = component_msg_iter
            .next()
            .expect("must have a first component msg");

        if let syn::GenericArgument::Type(type_) = first_arg {
            if let syn::Type::Path(type_path) = type_ {
                let path_segment: &syn::PathSegment = type_path
                    .path
                    .segments
                    .last()
                    .expect("must have a generic path segment");
                return (&path_segment.ident, path_segment);
            } else {
                panic!("expecting a type path");
            }
        } else {
            panic!("expecting a generic argument type");
        }
    } else {
        panic!("not an AngleBracketed");
    }
}
