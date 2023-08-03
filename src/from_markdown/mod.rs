use std::error::Error;

use cli_failure::bail;
use glob::glob;
use regex::Regex;
use rustdoc_types::Crate;
use tracing::{error, info, warn};
use xshell::{cmd, Shell};

use crate::consts::*;
use crate::get_crate_name;
use crate::FromMarkdown;

mod item_info;
use self::item_info::get_item_info;

mod item_path;

mod module_path;

mod supported_item;

mod update_docs;
use self::update_docs::update_docs;

pub const FROM_MARKDOWN_MARKER: &str = ".doc_sync_from_markdown";

#[wrap_match::wrap_match(disregard_result = true)]
pub fn from_markdown(
    sh: Shell,
    FromMarkdown {
        input_dir,
        allow_dirty,
    }: FromMarkdown,
) -> Result<(), Box<dyn Error>> {
    if {
        let git_output = cmd!(sh, "git status --short").output()?;
        let git_output = String::from_utf8_lossy(&git_output.stdout);
        !git_output.trim().is_empty()
    } {
        if allow_dirty {
            warn!("The current repository has uncommitted changes, but `--allow-dirty` was passed so doc-sync will continue.");
        } else {
            bail!("There are uncommitted changes. doc-sync may cause you to lose work. Therefore, it is recommended to commit your changes (or at least run `dura capture`) before using from-markdown. To bypass this warning, use the `--allow-dirty` command line argument.");
        }
    }

    info!("Getting crate name");
    let crate_name = get_crate_name(&sh)?;
    info!("Crate name is {crate_name}");

    let json_path = format!("./target/doc/{crate_name}.json");
    info!("Reading previously outputted JSON from {json_path}");
    let json = sh.read_file(&json_path)?;

    info!("Deserializing JSON");
    let json: Crate = serde_json::from_str(&json)?;

    info!("Going through input markdown files");
    let id_capture =
        Regex::new(&format!("{METADATA_ID_PREFIX}([^\"]*){METADATA_ID_SUFFIX}")).unwrap();
    for file in glob(&format!("{}/**/*.md", input_dir.display()))? {
        match file {
            Ok(file) => {
                info!(file = display(file.display()), "Found markdown file");
                if let Ok(Some(item)) = get_item_info(&sh, &json, &id_capture, &input_dir, file) {
                    update_docs(&sh, item);
                }
            }
            Err(e) => error!("Error when finding markdown file: {e:?}"),
        }
    }

    info!("Creating .doc_sync_from_markdown");
    sh.write_file(input_dir.join(FROM_MARKDOWN_MARKER), "This file is created to tell doc-sync's to-markdown subcommand that it's probably safe to overwrite the generated markdown files as a safeguard against losing work.")?;

    info!("Running rustfmt");
    cmd!(sh, "cargo fmt").run()?;

    Ok(())
}
