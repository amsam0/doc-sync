use rustdoc_types::ItemKind;
use syn::{
    spanned::Spanned, Field, ImplItemConst, ImplItemFn, ImplItemType, Item, ItemConst, ItemEnum,
    ItemExternCrate, ItemFn, ItemImpl, ItemMacro, ItemMod, ItemStatic, ItemStruct, ItemTrait,
    ItemTraitAlias, ItemType, ItemUnion, ItemUse, TraitItemConst, TraitItemFn, TraitItemType,
    Variant,
};
use tracing::error;

#[derive(Debug)]
pub enum SupportedItem<'lt> {
    // Items
    Const(&'lt ItemConst),
    Enum(&'lt ItemEnum),
    ExternCrate(&'lt ItemExternCrate),
    Fn(&'lt ItemFn),
    Impl(&'lt ItemImpl),
    Macro(&'lt ItemMacro),
    Module(&'lt ItemMod),
    Static(&'lt ItemStatic),
    Struct(&'lt ItemStruct),
    Trait(&'lt ItemTrait),
    TraitAlias(&'lt ItemTraitAlias),
    Type(&'lt ItemType),
    Union(&'lt ItemUnion),
    Use(&'lt ItemUse),

    // Implementation items
    ImplConst(&'lt ImplItemConst),
    ImplFn(&'lt ImplItemFn),
    ImplType(&'lt ImplItemType),

    // Trait items
    TraitConst(&'lt TraitItemConst),
    TraitFn(&'lt TraitItemFn),
    TraitType(&'lt TraitItemType),

    // Struct/enum
    Variant(&'lt Variant),
    Field(&'lt Field),
}
pub use SupportedItem::*;

impl<'lt> SupportedItem<'lt> {
    pub fn inner(&'lt self) -> &'lt dyn Spanned {
        match &self {
            // Items
            Const(i) => i,
            Enum(i) => i,
            ExternCrate(i) => i,
            Fn(i) => i,
            Impl(i) => i,
            Macro(i) => i,
            Module(i) => i,
            Static(i) => i,
            Struct(i) => i,
            Trait(i) => i,
            TraitAlias(i) => i,
            Type(i) => i,
            Union(i) => i,
            Use(i) => i,

            // Implementation items
            ImplConst(i) => i,
            ImplFn(i) => i,
            ImplType(i) => i,

            // Trait items
            TraitConst(i) => i,
            TraitFn(i) => i,
            TraitType(i) => i,

            // Struct/enum
            Variant(i) => i,
            Field(i) => i,
        }
    }
}

impl SupportedItem<'_> {
    #[rustfmt::skip]
    #[tracing::instrument]
    pub fn from_item<'lt>(item: &'lt Item, part: &String, kind: &ItemKind) -> Option<SupportedItem<'lt>> {
        match item {
            Item::Const(item) => if matches!(kind, ItemKind::Constant) && &item.ident.to_string() == part { Some(Const(item)) } else { None },
            Item::Enum(item) => if matches!(kind, ItemKind::Enum) && &item.ident.to_string() == part { Some(Enum(item)) } else { None },
            Item::ExternCrate(item) => if matches!(kind, ItemKind::ExternCrate) && &item.ident.to_string() == part { Some(ExternCrate(item)) } else { None },
            Item::Fn(item) => if matches!(kind, ItemKind::Function) && &item.sig.ident.to_string() == part { Some(Fn(item)) } else { None },
            Item::Impl(item) => if matches!(kind, ItemKind::Impl) { Some(Impl(item)) } else { None },
            Item::Macro(item) => if matches!(kind, ItemKind::Macro) && {
                if let Some(ident) = &item.ident {
                    &ident.to_string() == part
                } else {
                    false
                }
            } { Some(Macro(item)) } else { None },
            Item::Mod(item) => if matches!(kind, ItemKind::Module) && &item.ident.to_string() == part { Some(Module(item)) } else { None },
            Item::Static(item) => if matches!(kind, ItemKind::Static) && &item.ident.to_string() == part { Some(Static(item)) } else { None },
            Item::Struct(item) => if matches!(kind, ItemKind::Struct) && &item.ident.to_string() == part { Some(Struct(item)) } else { None },
            Item::Trait(item) => if matches!(kind, ItemKind::Trait) && &item.ident.to_string() == part { Some(Trait(item)) } else { None },
            Item::TraitAlias(item) => if matches!(kind, ItemKind::TraitAlias) && &item.ident.to_string() == part { Some(TraitAlias(item)) } else { None },
            Item::Type(item) => if matches!(kind, ItemKind::Typedef) && &item.ident.to_string() == part { Some(Type(item)) } else { None },
            Item::Union(item) => if matches!(kind, ItemKind::Union) && &item.ident.to_string() == part { Some(Union(item)) } else { None },
            Item::Use(item) => if matches!(kind, ItemKind::Import) { Some(Use(item)) } else { None },
            _ => {
                error!("Unknown or unsupported item");
                None
            }
        }
    }
}
