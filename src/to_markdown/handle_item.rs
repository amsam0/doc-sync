use std::error::Error;
use std::path::PathBuf;

use rustdoc_types::{Crate, Id, Item, ItemKind};
use tracing::{debug, warn};
use xshell::Shell;

use crate::consts::*;
use crate::edit_docs;

use super::iterate_children::iterate_children;

pub fn handle_item<'rustdoc>(
    sh: &Shell,
    json: &'rustdoc Crate,
    output_dir: &PathBuf,
    handled_ids: &mut Vec<&'rustdoc String>,
    id: &'rustdoc Id,
    item: &'rustdoc Item,
    path: &Vec<String>,
    kind: &ItemKind,
) -> Result<(), Box<dyn Error>> {
    if handled_ids.contains(&&id.0) {
        return Ok(());
    }

    let mut file_path = path.join("/");
    file_path.push_str(&format!("~{kind:?}.md"));

    let mut docs: Vec<_> = item
        .docs
        .clone()
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_owned())
        .collect();
    edit_docs::to_markdown(&mut docs);
    let mut docs = docs.join("\n");
    docs.insert_str(0, &format!("{METADATA_COMMENT_PREFIX}{METADATA_ID_PREFIX}{}{METADATA_ID_SUFFIX}{METADATA_COMMENT_SUFFIX}\n\n", id.0));

    if sh.path_exists(&file_path) {
        warn!(file_path, "Rare edge case has occurred! Somehow you have triggered a conflict. Two items have the same path, the currently existing file will be overwritten");
    }
    debug!(file_path, "Generating");
    sh.write_file(output_dir.join(file_path), docs)?;
    handled_ids.push(&id.0);

    iterate_children(sh, json, output_dir, handled_ids, path, &item.inner)?;

    Ok(())
}
