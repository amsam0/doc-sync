use std::error::Error;

use cli_failure::bail;
use rustdoc_types::ItemKind;
use syn::{ImplItem, Item, Stmt, TraitItem};

use crate::from_markdown::item_path::ItemPath;
use crate::from_markdown::supported_item::SupportedItem;

pub fn resolve_item<'lt>(
    candidates: &'lt Vec<Item>,
    item_path: ItemPath,
) -> Result<SupportedItem<'lt>, Box<dyn Error>> {
    let mut parent = None;
    for (part, kind) in item_path.iter() {
        if let Some(unwrapped_parent) = &parent {
            use SupportedItem::*;

            // Try to find children of the parent that matches the part and kind
            // unreachable = no children
            match *unwrapped_parent {
                // Items
                Const(_) => unreachable!(),
                Enum(unwrapped_parent) => {
                    if !matches!(kind, ItemKind::Variant) {
                        bail!("{part:?} was a {kind:?} but it should be a Variant since the parent was an enum");
                    }

                    match unwrapped_parent
                        .variants
                        .iter()
                        .find(|v| &v.ident.to_string() == part)
                    {
                        Some(v) => parent = Some(Variant(v)),
                        None => {
                            bail!(
                            "Couldn't find a variant with a name of {part:?}. Parent: {parent:#?}"
                        )
                        }
                    }
                }
                ExternCrate(_) => unreachable!(),
                Fn(unwrapped_parent) => {
                    match unwrapped_parent
                        .block
                        .stmts
                        .iter()
                        .filter_map(|s| {
                            if let Stmt::Item(item) = s {
                                Some(item)
                            } else {
                                None
                            }
                        })
                        .filter_map(|i| SupportedItem::from_item(i, part, kind))
                        .next()
                    {
                        Some(i) => parent = Some(i),
                        None => {
                            bail!(
                                "Couldn't get item for {part:?} with kind {kind:?}. Parent: {parent:#?}"
                            )
                        }
                    }
                }
                Impl(unwrapped_parent) => {
                    if !(matches!(kind, ItemKind::AssocConst)
                        || matches!(kind, ItemKind::Function)
                        || matches!(kind, ItemKind::AssocType))
                    {
                        bail!("{part:?} was a {kind:?} but it should be a AssocConst, Function or AssocType since the parent was an impl");
                    }

                    match unwrapped_parent
                        .items
                        .iter()
                        .filter_map(|item| {
                            match item {
                                #[rustfmt::skip]
                                ImplItem::Const(item) => if matches!(kind, ItemKind::AssocConst) && &item.ident.to_string() == part { Some(ImplConst(item)) } else { None },
                                ImplItem::Fn(item) => if matches!(kind, ItemKind::Function) && &item.sig.ident.to_string() == part { Some(ImplFn(item)) } else { None },
                                ImplItem::Type(item) => if matches!(kind, ItemKind::AssocType) && &item.ident.to_string() == part { Some(ImplType(item)) } else { None },
                                _ => None,
                            }
                        })
                        .next()
                    {
                        Some(i) => parent = Some(i),
                        None => bail!("Couldn't get item for {part:?} with kind {kind:?}. Parent: {parent:#?}"),
                    }
                }
                Macro(_) => unreachable!(),
                Module(unwrapped_parent) => {
                    let Some((_, ref items)) = unwrapped_parent.content else {
                        bail!("Couldn't get item for {part:?} with kind {kind:?} because the parent module had no content");
                    };

                    match items
                        .iter()
                        .filter_map(|i| SupportedItem::from_item(i, part, kind))
                        .next()
                    {
                        Some(i) => parent = Some(i),
                        None => {
                            bail!(
                                "Couldn't get item for {part:?} with kind {kind:?}. Parent: {parent:#?}"
                            )
                        }
                    };
                }
                Static(_) => unreachable!(),
                Struct(unwrapped_parent) => {
                    if !matches!(kind, ItemKind::StructField) {
                        bail!("{part:?} was a {kind:?} but it should be a StructField since the parent was a struct");
                    }

                    match unwrapped_parent.fields.iter().find(|f| {
                        if let Some(ident) = &f.ident {
                            &ident.to_string() == part
                        } else {
                            false
                        }
                    }) {
                        Some(f) => parent = Some(Field(f)),
                        None => {
                            bail!(
                            "Couldn't find a field with a name of {part:?}. Parent: {parent:#?}"
                        )
                        }
                    }
                }
                Trait(unwrapped_parent) => {
                    if !(matches!(kind, ItemKind::AssocConst)
                        || matches!(kind, ItemKind::Function)
                        || matches!(kind, ItemKind::AssocType))
                    {
                        bail!("{part:?} was a {kind:?} but it should be a AssocConst, Function or AssocType since the parent was an trait");
                    }

                    match unwrapped_parent
                        .items
                        .iter()
                        .filter_map(|item| {
                            match item {
                                #[rustfmt::skip]
                                TraitItem::Const(item) => if matches!(kind, ItemKind::AssocConst) && &item.ident.to_string() == part { Some(TraitConst(item)) } else { None },
                                TraitItem::Fn(item) => if matches!(kind, ItemKind::Function) && &item.sig.ident.to_string() == part { Some(TraitFn(item)) } else { None },
                                TraitItem::Type(item) => if matches!(kind, ItemKind::AssocType) && &item.ident.to_string() == part { Some(TraitType(item)) } else { None },
                                _ => None,
                            }
                        })
                        .next()
                    {
                        Some(i) => parent = Some(i),
                        None => bail!("Couldn't get item for {part:?} with kind {kind:?}. Parent: {parent:#?}"),
                    }
                }
                TraitAlias(_) => unreachable!(),
                Type(_) => unreachable!(),
                Union(_unwrapped_parent) => {
                    bail!("Couldn't find a field with a name of {part:?} because the parent is a union. Rustdoc doesn't seem to support union field documentation. If they are classified as struct fields, please create a GitHub Issue");
                }
                Use(_) => unreachable!(),

                // Implementation items
                ImplConst(_) => unreachable!(),
                ImplFn(unwrapped_parent) => {
                    match unwrapped_parent
                        .block
                        .stmts
                        .iter()
                        .filter_map(|s| {
                            if let Stmt::Item(item) = s {
                                Some(item)
                            } else {
                                None
                            }
                        })
                        .filter_map(|i| SupportedItem::from_item(i, part, kind))
                        .next()
                    {
                        Some(i) => parent = Some(i),
                        None => {
                            bail!(
                                "Couldn't get item for {part:?} with kind {kind:?}. Parent: {parent:#?}"
                            )
                        }
                    }
                }
                ImplType(_) => unreachable!(),

                // Trait items
                TraitConst(_) => unreachable!(),
                TraitFn(unwrapped_parent) => {
                    let Some(ref block) = unwrapped_parent.default else {
                        bail!("Couldn't get item for {part:?} with kind {kind:?} because the parent trait function has no default");
                    };

                    match block
                        .stmts
                        .iter()
                        .filter_map(|s| {
                            if let Stmt::Item(item) = s {
                                Some(item)
                            } else {
                                None
                            }
                        })
                        .filter_map(|i| SupportedItem::from_item(i, part, kind))
                        .next()
                    {
                        Some(i) => parent = Some(i),
                        None => {
                            bail!(
                                "Couldn't get item for {part:?} with kind {kind:?}. Parent: {parent:#?}"
                            )
                        }
                    }
                }
                TraitType(_) => unreachable!(),

                // Struct/enum
                Variant(_unwrapped_parent) => {
                    bail!("Couldn't find a field with a name of {part:?} because the parent is an enum variant. Rustdoc doesn't seem to support enum variant field documentation. If they are classified as struct fields, please create a GitHub Issue");
                }
                Field(_) => unreachable!(),
            }
        } else {
            match candidates
                .iter()
                .filter_map(|i| SupportedItem::from_item(i, part, kind))
                .next()
            {
                Some(i) => parent = Some(i),
                None => bail!("Couldn't get item for {part:?} with kind {kind:?}"),
            };
        }
    }

    let final_item = parent.unwrap(); // Should be safe as we will return an error if we can't find it
    Ok(final_item)
}
