use std::error::Error;
use std::path::PathBuf;

use rustdoc_types::{Crate, Id, ItemEnum, VariantKind};
use tracing::trace;
use xshell::Shell;

use super::handle_item::handle_item;
use super::item_enum_ext::ItemEnumExt;

pub fn iterate_children<'rustdoc>(
    sh: &Shell,
    json: &'rustdoc Crate,
    output_dir: &PathBuf,
    handled_ids: &mut Vec<&'rustdoc String>,
    parent_path: &Vec<String>,
    item: &'rustdoc ItemEnum,
) -> Result<(), Box<dyn Error>> {
    match item {
        ItemEnum::Module(item) => iterate_ids(
            sh,
            json,
            output_dir,
            handled_ids,
            parent_path,
            item.items.iter(),
        ),
        ItemEnum::ExternCrate { .. } => Ok(()),
        ItemEnum::Import(_) => Ok(()),

        ItemEnum::Union(item) => {
            iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                item.fields.iter(),
            )?;
            iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                item.impls.iter(),
            )
        }
        ItemEnum::Struct(item) => iterate_ids(
            sh,
            json,
            output_dir,
            handled_ids,
            parent_path,
            item.impls.iter(),
        ),
        ItemEnum::StructField(_) => Ok(()),
        ItemEnum::Enum(item) => {
            iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                item.variants.iter(),
            )?;
            iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                item.impls.iter(),
            )
        }
        ItemEnum::Variant(item) => match &item.kind {
            VariantKind::Plain => Ok(()),
            VariantKind::Tuple(fields) => iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                fields.iter().filter_map(|f| f.as_ref()),
            ),
            VariantKind::Struct { fields, .. } => iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                fields.iter(),
            ),
        },

        ItemEnum::Function(_) => Ok(()),

        ItemEnum::Trait(item) => {
            iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                item.items.iter(),
            )?;
            iterate_ids(
                sh,
                json,
                output_dir,
                handled_ids,
                parent_path,
                item.implementations.iter(),
            )
        }
        ItemEnum::TraitAlias(_) => Ok(()),
        ItemEnum::Impl(item) => iterate_ids(
            sh,
            json,
            output_dir,
            handled_ids,
            parent_path,
            item.items.iter(),
        ),

        ItemEnum::Typedef(_) => Ok(()),
        ItemEnum::OpaqueTy(_) => Ok(()),
        ItemEnum::Constant(_) => Ok(()),

        ItemEnum::Static(_) => Ok(()),

        ItemEnum::ForeignType => Ok(()),

        ItemEnum::Macro(_) => Ok(()),
        ItemEnum::ProcMacro(_) => Ok(()),

        ItemEnum::Primitive(_) => Ok(()),

        ItemEnum::AssocConst { .. } => Ok(()),
        ItemEnum::AssocType { .. } => Ok(()),
    }
}

fn iterate_ids<'rustdoc>(
    sh: &Shell,
    json: &'rustdoc Crate,
    output_dir: &PathBuf,
    handled_ids: &mut Vec<&'rustdoc String>,
    parent_path: &Vec<String>,
    ids: impl Iterator<Item = &'rustdoc Id>,
) -> Result<(), Box<dyn Error>> {
    for id in ids {
        let item = json.index.get(id).expect("rustdoc JSON output is invalid?");
        let kind = item.inner.to_item_kind();
        let Some(name) = item.name.to_owned() else {
            trace!(
                item_kind = debug(kind),
                span = debug(&item.span),
                parent = debug(parent_path),
                id = &id.0,
                "Skipping child item because it doesn't have a name"
            );
            continue;
        };
        let mut path = parent_path.clone();
        path.push(name);

        handle_item(sh, json, output_dir, handled_ids, id, item, &path, &kind)?;
    }

    Ok(())
}
