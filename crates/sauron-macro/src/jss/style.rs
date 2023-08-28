use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Lit, Result, Token};

/// ```ignore
/// background_color: "red",
/// border: (px(1), "solid", "green"),
/// ```
pub(crate) struct Style {
    properties: Vec<(Option<Annotation>, Property)>,
}

pub(crate) struct Annotation {
    punct: Token![#],
    group: proc_macro2::Group,
}

/// key value pair of a style
/// `background_color: "red"`
/// `border: (px(1), "solid", "green")`
struct Property {
    property: PropertyName,
    value: Expr,
}

/// `border`
/// `"background-color"`
struct PropertyName(String);

impl Parse for Style {
    /// ```ignore
    /// $(<property> : <pair>,)*
    /// ```
    fn parse(input: ParseStream) -> Result<Self> {
        let mut properties = vec![];
        while !input.is_empty() {
            // must be attribute annotations
            let anotation = if input.peek(Token![#]) {
                Some(input.parse()?)
            } else {
                None
            };
            let kv = input.parse()?;
            if input.peek(Token![,]) && !input.peek2(syn::token::Brace) {
                input.parse::<Token![,]>()?;
            }
            properties.push((anotation, kv));
        }
        Ok(Self { properties })
    }
}

impl ToTokens for Style {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let pretty_tokens = self.to_tokens_with_pretty(false);
        tokens.extend(pretty_tokens);
    }
}

impl Style {
    pub(crate) fn to_attr_tokens(&self) -> proc_macro2::TokenStream {
        let style_tokens = self.to_token_stream();
        quote! {
            sauron::html::attributes::attr("style", #style_tokens)
        }
    }

    pub(crate) fn to_tokens_with_pretty(&self, use_pretty: bool) -> proc_macro2::TokenStream {
        let expanded_properties: Vec<_> = self
            .properties
            .iter()
            .map(|(anotation, pair)| {
                let pair = pair.to_tokens_with_pretty(use_pretty);
                quote! { #anotation #pair,}
            })
            .collect();

        let properties_tokens = proc_macro2::TokenStream::from_iter(expanded_properties);

        let separator = if use_pretty { "\n" } else { "" };

        quote! {
            [#properties_tokens].join(#separator)
        }
    }
}

impl Parse for Annotation {
    fn parse(input: ParseStream) -> Result<Self> {
        let punct = input.parse::<Token![#]>()?;
        let group: proc_macro2::Group = input.parse()?;
        Ok(Self { punct, group })
    }
}

impl ToTokens for Annotation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let punct = &self.punct;
        let group = &self.group;
        tokens.extend(quote! {#punct #group});
    }
}

impl Parse for Property {
    ///
    /// ```ignore
    /// <property> : <expr>
    /// ```
    fn parse(input: ParseStream) -> Result<Self> {
        let property = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;
        Ok(Property { property, value })
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let pretty_tokens = self.to_tokens_with_pretty(false);
        tokens.extend(pretty_tokens);
    }
}

impl Property {
    fn to_tokens_with_pretty(&self, use_pretty: bool) -> proc_macro2::TokenStream {
        let property = &self.property;
        let value = &self.value;

        let value_expanded = quote! {
            sauron::html::attributes::Value::from(#value).to_string()
        };
        if use_pretty {
            quote! {
                format!("  {}: {};", #property, #value_expanded)
            }
        } else {
            quote! {
                format!("{}:{};", #property, #value_expanded)
            }
        }
    }
}

impl Parse for PropertyName {
    /// ```ignore
    /// "<literal>" | ident
    /// ```
    fn parse(input: ParseStream) -> Result<Self> {
        let property_name = if let Ok(ident) = input.parse::<Ident>() {
            ident.to_string()
        } else if let Ok(Lit::Str(v)) = input.parse::<Lit>() {
            v.value()
        } else {
            return Err(syn::Error::new(
                input.span(),
                format!("Expecting a property, found: \n\t{}", input.to_string()),
            ));
        };
        match sauron_core::html::lookup::match_property(&property_name) {
            Some(matched) => Ok(PropertyName(matched.to_string())),
            None => Err(syn::Error::new(
                input.span(),
                format!("unknown property: {property_name}"),
            )),
        }
    }
}

impl ToTokens for PropertyName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let property = &self.0;
        tokens.extend(quote! {#property});
    }
}
