use std::borrow::Cow;
use std::error::Error;
use std::path::{Component, PathBuf};

use cli_failure::{bail, failure_raw};
use normalize_path::NormalizePath;
use regex::Regex;
use rustdoc_types::{Crate, Id};
use tracing::{debug, info, warn};
use xshell::Shell;

use crate::consts::*;
use crate::edit_docs;
use crate::from_markdown::module_path::AbsoluteModulePath;

use super::item_path::ItemPath;
use super::module_path::RelativeModulePath;

#[derive(custom_debug_derive::Debug)]
pub struct ItemInfo<'lt> {
    pub id: String,
    #[debug(skip)]
    pub rustdoc_item: &'lt rustdoc_types::Item,
    #[debug(skip)]
    pub new_docs: Vec<String>,
    pub file_path: String,
    pub file_module_path: RelativeModulePath,
    pub item_path: ItemPath,
}

#[wrap_match::wrap_match(log_success = false)]
#[tracing::instrument(skip(sh, json, id_capture))]
pub fn get_item_info<'lt>(
    sh: &Shell,
    json: &'lt Crate,
    id_capture: &Regex,
    input_dir: &PathBuf,
    file: PathBuf,
) -> Result<Option<ItemInfo<'lt>>, Box<dyn Error>> {
    let new_docs = sh.read_file(&file)?;
    let captures = id_capture
        .captures(&new_docs)
        .ok_or_else(|| failure_raw!("No rustdoc ID found"))?;
    if captures.len() <= 1 {
        bail!("No rustdoc ID found");
    } else if captures.len() > 2 {
        warn!("More than one rustdoc ID found, only the first ID will be used");
    }

    let id = Id(captures[1].to_owned());
    debug!(id = id.0);
    let rustdoc_item = json.index.get(&id).ok_or_else(|| {
        failure_raw!(
            "No item found; did you re-run rustdoc in JSON output format since using to-markdown?"
        )
    })?;
    let mut new_docs: Vec<_> = new_docs
        .replace(&format!("{METADATA_COMMENT_PREFIX}{METADATA_ID_PREFIX}{}{METADATA_ID_SUFFIX}{METADATA_COMMENT_SUFFIX}", id.0), "")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();
    if new_docs
        == rustdoc_item
            .docs
            .to_owned()
            .unwrap_or_default()
            .trim()
            .lines()
            .collect::<Vec<_>>()
    {
        info!("Docs have not been changed");
        return Ok(None);
    }
    edit_docs::from_markdown(&mut new_docs);
    let file_path = rustdoc_item
        .span
        .as_ref()
        .ok_or_else(|| failure_raw!("No span for item"))?
        .filename
        .display()
        .to_string();
    let file_module_path = RelativeModulePath::from_file_path(&file_path);
    let module_path = if let Some(rustdoc_item_summary) = json.paths.get(&id) {
        Cow::Borrowed(&rustdoc_item_summary.path)
    } else {
        let mut module_path = vec![];
        for component in file
            .normalize()
            .strip_prefix(input_dir.normalize())
            .expect("file should always start with input_dir")
            .components()
        {
            if let Component::Normal(component) = component {
                let component = component.to_str().expect("path not utf-8");
                if component.ends_with(".md") {
                    module_path.push(component.split('~').next().unwrap().to_owned());
                    break;
                } else {
                    module_path.push(component.to_owned());
                }
            }
        }
        warn!(module_path = debug(&module_path), "Item seems to be an inner item. This means that we had to estimate the module path based on the file path, so it may be incorrect");
        Cow::Owned(module_path)
    };
    let item_path = ItemPath::new(
        &file_module_path,
        AbsoluteModulePath(&module_path),
        &json,
    )
    .ok_or_else(|| failure_raw!("Couldn't find item for part (see above); did you re-run rustdoc in JSON output format since using to-markdown?"))?;

    Ok(Some(ItemInfo {
        id: id.0,
        rustdoc_item,
        new_docs,
        file_path,
        file_module_path,
        item_path,
    }))
}
