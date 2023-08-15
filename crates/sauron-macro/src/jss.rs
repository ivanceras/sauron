use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Result, Token};

pub(crate) use style::Style;

mod style;

pub(crate) struct StyleSheetWithConditionalGroup {
    conditional_group: Expr,
    selector_with_styles: Vec<SelectorWithStyle>,
}

pub(crate) struct StyleSheet {
    selector_with_styles: Vec<SelectorWithStyle>,
}

/// ```ignore
/// ".layer0" : {
///     background_color: "red",
///     border: (px(1), "solid", "green"),
/// }
/// ```
struct SelectorWithStyle {
    selector: Expr,
    style: Style,
}

impl Parse for StyleSheet {
    /// $(<selector> : { <style> } (,)?)*
    fn parse(input: ParseStream) -> Result<Self> {
        let mut selector_with_styles = vec![];
        while !input.is_empty() {
            selector_with_styles.push(input.parse()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self {
            selector_with_styles,
        })
    }
}

impl ToTokens for StyleSheet {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expanded_selector_with_styles: Vec<_> = self
            .selector_with_styles
            .iter()
            .map(|ss| {
                quote! {
                    #ss,
                }
            })
            .collect();
        let ss_tokens = proc_macro2::TokenStream::from_iter(expanded_selector_with_styles);
        tokens.extend(quote! {
            [#ss_tokens].join("\n")
        });
    }
}

impl Parse for StyleSheetWithConditionalGroup {
    /// <conditional> : { $(<selector> : { <style> } (,)?)* }
    fn parse(input: ParseStream) -> Result<Self> {
        let conditional_group = input.parse()?;
        input.parse::<Token![:]>()?;
        let inner;
        syn::braced!(inner in input);
        let mut selector_with_styles = vec![];
        while !inner.is_empty() {
            let selector_with_style = inner.parse()?;
            selector_with_styles.push(selector_with_style);
            if inner.peek(Token![,]) {
                inner.parse::<Token![,]>()?;
            }
        }
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(Self {
            conditional_group,
            selector_with_styles,
        })
    }
}

impl ToTokens for StyleSheetWithConditionalGroup {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let conditional_group = &self.conditional_group;
        let expanded_selector_with_styles: Vec<_> = self
            .selector_with_styles
            .iter()
            .map(|ss| {
                quote! {
                    #ss,
                }
            })
            .collect();
        let ss_tokens = proc_macro2::TokenStream::from_iter(expanded_selector_with_styles);
        tokens.extend(quote! {
            format!("{} {{\n{}\n}}\n", #conditional_group, [#ss_tokens].join("\n"))
        });
    }
}

impl Parse for SelectorWithStyle {
    /// ```ignore
    /// <expr> : { <style> }
    /// ```
    fn parse(input: ParseStream) -> Result<Self> {
        let selector = input.parse::<Expr>()?;
        input.parse::<Token![:]>()?;
        let inner;
        syn::braced!(inner in input);
        let style = inner.parse()?;
        Ok(Self { selector, style })
    }
}

impl ToTokens for SelectorWithStyle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let selector = &self.selector;
        let style = &self.style.to_tokens_with_pretty(true);
        tokens.extend(quote! {
            format!("{} {{\n{}\n}}\n", #selector, #style)
        });
    }
}
