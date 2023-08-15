use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Lit, Result, Token};

/// ```ignore
/// background_color: "red",
/// border: (px(1), "solid", "green"),
/// ```
pub(crate) struct Style {
    properties: Vec<Property>,
}

/// key value pair of a style
/// `background_color: "red"`
/// `border: (px(1), "solid", "green")`
struct Property {
    property: IdentOrString,
    value: Expr,
}

/// `border`
/// `"background-color"`
enum IdentOrString {
    Ident(Ident),
    String(String),
}

impl Parse for Style {
    /// ```ignore
    /// $(<property> : <pair>,)*
    /// ```
    fn parse(input: ParseStream) -> Result<Self> {
        let mut properties = vec![];
        while !input.is_empty() {
            let kv = input.parse()?;
            if input.peek(Token![,]) && !input.peek2(syn::token::Brace) {
                input.parse::<Token![,]>()?;
            }
            properties.push(kv);
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
            .map(|pair| {
                let pair = pair.to_tokens_with_pretty(use_pretty);
                quote! {#pair,}
            })
            .collect();

        let properties_tokens = proc_macro2::TokenStream::from_iter(expanded_properties);

        let separator = if use_pretty { "\n" } else { "" };

        quote! {
            [#properties_tokens].join(#separator)
        }
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

impl Parse for IdentOrString {
    /// ```ignore
    /// "<literal>" | ident
    /// ```
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(ident) = input.parse::<Ident>() {
            Ok(Self::Ident(ident))
        } else if let Ok(Lit::Str(v)) = input.parse::<Lit>() {
            Ok(Self::String(v.value()))
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expecting an identifier or a string literal",
            ))
        }
    }
}

impl ToTokens for IdentOrString {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expanded = match self {
            Self::Ident(ident) => {
                let ident = ident.to_string();
                let property = sauron_core::html::lookup::match_property(&ident);
                quote! {#property}
            }
            Self::String(v) => {
                let property = sauron_core::html::lookup::match_property(&v);
                quote! {#property}
            }
        };
        tokens.extend(expanded);
    }
}
