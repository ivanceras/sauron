use proc_macro2::{
    Span,
    TokenStream,
};
use quote::ToTokens;
use syn::{
    ext::IdentExt as _,
    parse::{
        Parse,
        ParseStream,
    },
    token,
    Error,
    Expr,
    ExprForLoop,
    Ident,
    Lit,
    LitStr,
    Token,
};

pub(super) struct Node {
    name: Ident,
    attrs: Vec<Attribute>,
    children: Vec<Child>,
}

impl Node {
    /// Parse an opening node.
    fn parse_open(
        input: ParseStream,
    ) -> syn::Result<(bool, Ident, Vec<Attribute>)> {
        input.parse::<Token![<]>()?;
        let element = input.parse::<Ident>()?;

        let mut attrs = Vec::new();

        while !input.is_empty() {
            if input.peek(Token![>]) {
                input.parse::<Token![>]>()?;
                return Ok((true, element, attrs));
            }

            if input.peek(Token![/]) && input.peek2(Token![>]) {
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;
                return Ok((false, element, attrs));
            }

            attrs.push(input.parse()?);
        }

        Err(input.error(format!("Expected closing of element `{}`", element)))
    }

    fn children_to_tokens(&self, receiver: &Ident, tokens: &mut TokenStream) {
        if !self.children.is_empty() {
            let count = self.children.len();

            tokens.extend(quote::quote! {
                let mut #receiver = Vec::with_capacity(#count);
            });

            for c in &self.children {
                match c {
                    Child::Node(node) => {
                        tokens.extend(quote::quote! {
                            #receiver.push(#node);
                        });
                    }
                    Child::LitStr(s) => {
                        tokens.extend(quote::quote! {
                            #receiver.push(sauron::Node::Text(String::from(#s)));
                        });
                    }
                    Child::Eval(e) => {
                        tokens.extend(quote::quote! {
                            #receiver.push(sauron::Node::from(#e));
                        });
                    }
                    Child::Loop(ExprForLoop {
                        pat, expr, body, ..
                    }) => {
                        tokens.extend(quote::quote! {
                            for #pat in #expr {
                                #receiver.push(sauron::Node::from(#body));
                            }
                        });
                    }
                }
            }
        } else {
            tokens.extend(quote::quote! {
                let #receiver = Vec::new();
            });
        }
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut children = Vec::new();

        if !input.peek(Token![<]) {
            return Err(input.error("expected opening caret"));
        }

        let (is_open, name, attrs) = Self::parse_open(input)?;

        if is_open {
            loop {
                // Parse end node.
                if input.peek(Token![<]) && input.peek2(Token![/]) {
                    input.parse::<Token![<]>()?;
                    input.parse::<Token![/]>()?;
                    let end = input.parse::<Ident>()?;
                    input.parse::<Token![>]>()?;

                    if name != end {
                        return Err(Error::new(
                            end.span(),
                            format!(
                                "Closing node `{}` does not match open node `{}`",
                                end, name
                            ),
                        ));
                    }

                    break;
                }

                if input.is_empty() {
                    return Err(input.error(format!(
                        "Expected closing of element `{}`",
                        name
                    )));
                }

                if input.peek(LitStr) {
                    children.push(Child::LitStr(input.parse()?));
                } else if input.peek(token::Brace) {
                    let content;
                    let _ = syn::braced!(content in input);

                    let child = if content.peek(Token![for]) {
                        let for_loop = content.parse::<ExprForLoop>()?;
                        Child::Loop(for_loop)
                    } else {
                        Child::Eval(content.parse()?)
                    };

                    children.push(child);
                } else {
                    children.push(Child::Node(input.parse()?));
                }
            }
        }

        Ok(Node {
            name,
            attrs,
            children,
        })
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = LitStr::new(&self.name.to_string(), self.name.span());
        let attrs = &self.attrs;

        let receiver = Ident::new("children", Span::call_site());
        let mut children_tokens = TokenStream::new();
        self.children_to_tokens(&receiver, &mut children_tokens);

        tokens.extend(quote::quote! {{
            let attrs = vec![#(#attrs),*];
            #children_tokens
            sauron::html::html_element(#name, attrs, children)
        }});
    }
}

enum Attribute {
    Event {
        name: Ident,
        value: Expr,
    },
    /// A literal attribute.
    ///
    /// Like `<button style="border: 1px solid red;">`.
    Lit {
        name: LitStr,
        value: Lit,
    },
    /// An expression attribute.
    ///
    /// The expression is expected to evaluate to a attribute `Value`.
    ///
    /// Like `<button style={style()}>`.
    Expr {
        name: LitStr,
        value: Expr,
    },
    /// An empty attribute.
    ///
    /// Like `<button disabled>`.
    Empty {
        name: LitStr,
    },
    /// An expression expected to generate an attribute.
    AttributeExpr {
        value: Expr,
    },
}

impl Attribute {
    /// Convert a name `n` from lower_camel to kebab-case.
    fn convert_name(n: &str) -> String {
        let mut out = String::with_capacity(n.len());

        for c in n.trim_matches('_').chars() {
            match c {
                '_' => out.push('-'),
                c => out.push(c),
            }
        }

        out
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(token::Brace) {
            let content;
            let _ = syn::braced!(content in input);
            let value = content.parse()?;
            return Ok(Self::AttributeExpr { value });
        }

        let name = input.parse::<Ident>()?.unraw();

        let event = match name.to_string().as_str() {
            n if n.starts_with("on_") => true,
            _ => false,
        };

        if event {
            input.parse::<Token![=]>()?;

            let content;
            let _ = syn::braced!(content in input);

            let value = content.parse()?;
            Ok(Self::Event { name, value })
        } else {
            let name = LitStr::new(
                &Self::convert_name(&name.to_string()),
                name.span(),
            );

            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;

                if input.peek(token::Brace) {
                    let content;
                    let _ = syn::braced!(content in input);
                    let value = content.parse()?;
                    Ok(Self::Expr { name, value })
                } else {
                    let value = input.parse()?;
                    Ok(Self::Lit { name, value })
                }
            } else {
                Ok(Self::Empty { name })
            }
        }
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lit { name, value } => {
                tokens.extend(quote::quote! {
                    sauron::Attribute::new(
                        None,
                        #name,
                        sauron::html::attributes::AttributeValue::Simple(
                            sauron::html::attributes::Value::from(#value)
                        )
                    )
                });
            }
            Self::Expr { name, value } => {
                tokens.extend(quote::quote! {
                    sauron::Attribute::new(
                        None,
                        #name,
                        sauron::html::attributes::AttributeValue::Simple(
                            sauron::html::attributes::Value::from(#value)
                        )
                    )
                });
            }
            Self::Empty { name } => {
                tokens.extend(quote::quote! {
                    sauron::Attribute::new(
                        None,
                        #name,
                        sauron::html::attributes::AttributeValue::Empty,
                    )
                });
            }
            Self::Event { name, value } => {
                tokens.extend(quote::quote! {
                    sauron::events::#name(#value)
                });
            }
            Self::AttributeExpr { value } => {
                tokens.extend(quote::quote! {
                    sauron::Attribute::from(#value)
                });
            }
        }
    }
}

enum Child {
    Node(Node),
    LitStr(LitStr),
    Eval(Expr),
    Loop(ExprForLoop),
}
