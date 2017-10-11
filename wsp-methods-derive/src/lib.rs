#![allow(deprecated)]
#![feature(quote)]
#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::ast::{ItemKind, ImplItemKind, MetaItem, Name, PatKind, FunctionRetTy};
use syntax::tokenstream::{TokenStream, TokenTree, Delimited};
use syntax::parse::token::{Token, DelimToken};
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::ext::quote::rt::Span;
use syntax::feature_gate::AttributeType;
use rustc_plugin::Registry;

use std::collections::HashSet;
use std::ops::Deref;

fn type_list(tokens: TokenStream) -> Option<Vec<String>> {
    let mut list = Vec::new();

    let mut comma_chk = false;
    for tree in tokens.trees() {
        match tree {
            TokenTree::Token(_, tok) => {
                match tok {
                    Token::Ident(id) => {
                        if comma_chk {return None;}
                        comma_chk = true;
                        list.push(String::from(&*id.name.as_str()));
                    }
                    Token::Comma => {
                        if !comma_chk {return None;}
                        comma_chk = false;
                    }
                    _ => {
                        return None;
                    }
                }
            }
            TokenTree::Delimited(..) => {
                return None;
            }
        }
    }
    Some(list)
}

fn parse_methods(
    exc: &mut ExtCtxt,
    _sp: Span,
    _meta_item: &MetaItem,
    item: &Annotatable,
    _push: &mut FnMut(Annotatable),
) {
    if let &Annotatable::Item(ref item) = item {
        if let ItemKind::Impl(
            ref _safety,
            ref _polarity,
            ref _default,
            ref _generics,
            ref _traits,
            ref types,
            ref items,
        ) = item.node
        {
            let mut type_names = HashSet::new();

            for method in items
                .iter()
                .filter(|i| i.attrs.iter().find(|a| a.path == "WSPMethod").is_some())
            {
                if let ImplItemKind::Method(ref sig, ref block) = method.node {
                    for arg in &sig.decl.inputs {
                        if let PatKind::Ident(ref _mode, ref ident, ref _pat) = arg.pat.node {
                            type_names.insert(arg.ty.clone());
                        }
                    }

                    if let FunctionRetTy::Ty(ref ty) = sig.decl.output {
                        type_names.insert(ty.clone());
                    }
                }

                for tree in method.attrs.iter().find(|a| a.path == "WSPMethod").unwrap().clone().tokens.trees() {
                    if let TokenTree::Delimited(.., Delimited{delim: DelimToken::Paren, tts: tts}) = tree {
                        for ty in type_list(TokenStream::from(tts)).unwrap() {
                            println!("{:#?}", ty);
                        }
                    }
                }
            }
            let mut stmts = Vec::new();
            stmts.push(quote_stmt!(exc, let mut map = serde_json::Map::new();));
            for name in type_names {
                println!("{:?}", name.to_source());
                stmts.push(quote_stmt!(exc, 
                    map.insert(<$name as WSPType>::get_name(), <$name as WSPType>::get_type());
                ));
            }
            stmts.push(quote_stmt!(exc, serde_json::Value::Object(map)));

            let decl = quote_item!(exc,
                impl WSPMethods for $types {
                    fn get_types() -> serde_json::Value {
                        $stmts
                    }
                    fn get_methods() -> serde_json::Value {
                        unimplemented!();
                    }
                }
            );
            // println!("{:#?}", decl);
            //_push(Annotatable::Item(decl.unwrap()));
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_custom_derive(
        Name::intern("derive_WSPMethods"),
        SyntaxExtension::MultiDecorator(Box::new(parse_methods)),
    );
    reg.register_attribute(String::from("WSPMethod"), AttributeType::Whitelisted);
}
