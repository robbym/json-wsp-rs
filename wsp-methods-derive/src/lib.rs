#![allow(deprecated)]
#![feature(box_patterns)]
#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::ast::{ItemKind, MetaItem, Name};
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::ext::quote::rt::Span;
use syntax::feature_gate::AttributeType;
use rustc_plugin::Registry;

fn parse_methods(
    _exc: &mut ExtCtxt,
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
            ref _types,
            ref items,
        ) = item.node
        {
            for method in items
                .iter()
                .filter(|i| i.attrs.iter().find(|a| a.path == "WSPMethod").is_some())
            {
                println!("{:#?}", method.ident);
            }
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
