// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Ident, ItemMod, LitStr, Result, Token, Visibility,
};

#[derive(Clone, PartialEq, Eq)]
pub struct CxxQtItemMod {
    pub inner: ItemMod,
    pub extra: i32,
}

impl std::fmt::Debug for CxxQtItemMod {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut formatter = formatter.debug_tuple("ItemMod");
        formatter.field(&self.inner);
        formatter.finish()
    }
}

impl std::ops::Deref for CxxQtItemMod {
    type Target = ItemMod;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for CxxQtItemMod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Parse for CxxQtItemMod {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let mod_token: Token![mod] = input.parse()?;
        let ident: Ident = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![;]) {
            Ok(Self {
                inner: ItemMod {
                    attrs,
                    vis,
                    mod_token,
                    ident,
                    content: None,
                    semi: Some(input.parse()?),
                },
                extra: 0,
            })
        } else if lookahead.peek(syn::token::Brace) {
            let mut extra = 0;
            let content;
            let brace_token = syn::braced!(content in input);
            attrs.append(&mut content.call(Attribute::parse_inner)?);

            let mut items = Vec::new();
            while !content.is_empty() {
                if content.peek(Token![extern]) {
                    let ahead = content.fork();
                    let _attrs = ahead.call(Attribute::parse_outer)?;
                    let _extern_token: Token![extern] = ahead.parse()?;
                    let name: Option<LitStr> = ahead.parse()?;

                    if let Some(name) = name {
                        if name.value() == "Qt" {
                            // Parse our extern "Qt" block
                            //
                            // TODO: here we need to build our extra Qt items
                            let _attrs = content.call(Attribute::parse_outer)?;
                            let _extern_token: Token![extern] = content.parse()?;
                            let _name: LitStr = content.parse()?;
                            let inner;
                            let _brace_token = syn::braced!(inner in content);

                            // Move the cursor past all remaining tokens, otherwise parse2 fails
                            //
                            // TODO: here we need to build our extra Qt items
                            inner.step(|cursor| {
                                let mut rest = *cursor;
                                while let Some((_, next)) = rest.token_tree() {
                                    rest = next;
                                }
                                Ok(((), rest))
                            })?;

                            extra += 1;
                            continue;
                        }
                    }
                }

                items.push(content.parse()?);
            }

            Ok(Self {
                inner: ItemMod {
                    attrs,
                    vis,
                    mod_token,
                    ident,
                    content: Some((brace_token, items)),
                    semi: None,
                },
                extra,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for CxxQtItemMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.inner.to_tokens(tokens);
    }
}
