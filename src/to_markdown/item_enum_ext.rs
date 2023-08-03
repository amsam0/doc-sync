use rustdoc_types::{ItemEnum, ItemKind, MacroKind, ProcMacro};

#[easy_ext::ext(ItemEnumExt)]
pub impl ItemEnum {
    fn to_item_kind(&self) -> ItemKind {
        match &self {
            ItemEnum::Module(_) => ItemKind::Module,
            ItemEnum::ExternCrate { .. } => ItemKind::ExternCrate,
            ItemEnum::Import(_) => ItemKind::Import,

            ItemEnum::Union(_) => ItemKind::Union,
            ItemEnum::Struct(_) => ItemKind::Struct,
            ItemEnum::StructField(_) => ItemKind::StructField,
            ItemEnum::Enum(_) => ItemKind::Enum,
            ItemEnum::Variant(_) => ItemKind::Variant,

            ItemEnum::Function(_) => ItemKind::Function,

            ItemEnum::Trait(_) => ItemKind::Trait,
            ItemEnum::TraitAlias(_) => ItemKind::TraitAlias,
            ItemEnum::Impl(_) => ItemKind::Impl,

            ItemEnum::Typedef(_) => ItemKind::Typedef,
            ItemEnum::OpaqueTy(_) => ItemKind::OpaqueTy,
            ItemEnum::Constant(_) => ItemKind::Constant,

            ItemEnum::Static(_) => ItemKind::Static,

            ItemEnum::ForeignType => ItemKind::ForeignType,

            ItemEnum::Macro(_) => ItemKind::Macro,
            ItemEnum::ProcMacro(ProcMacro { kind, .. }) => match kind {
                MacroKind::Bang => ItemKind::Macro,
                MacroKind::Attr => ItemKind::ProcAttribute,
                MacroKind::Derive => ItemKind::ProcDerive,
            },

            ItemEnum::Primitive(_) => ItemKind::Primitive,

            ItemEnum::AssocConst { .. } => ItemKind::AssocConst,
            ItemEnum::AssocType { .. } => ItemKind::AssocType,
        }
    }
}
