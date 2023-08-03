use std::error::Error;

use cli_failure::bail;
use rustdoc_types::Crate;
use tracing::{info, warn};
use xshell::{cmd, Shell};

use crate::from_markdown::FROM_MARKDOWN_MARKER;
use crate::get_crate_name;
use crate::ToMarkdown;

mod handle_item;
use self::handle_item::handle_item;

mod item_enum_ext;

mod iterate_children;

#[wrap_match::wrap_match(disregard_result = true)]
pub fn to_markdown(
    sh: Shell,
    ToMarkdown {
        cargo_command,
        cargo_doc_arguments,
        rustdoc_arguments,
        output_dir,
        force,
    }: ToMarkdown,
) -> Result<(), Box<dyn Error>> {
    if sh.path_exists(&output_dir) {
        if !sh.path_exists(&output_dir.join(FROM_MARKDOWN_MARKER)) {
            if force {
                warn!(".doc_sync_from_markdown does not exist in the output directory, but `--force` was passed so doc-sync will continue.");
            } else {
                bail!("You have not yet converted the previously generated markdown files back into doc comments. Please manually delete the output directory or pass `--force` to force overwriting the output directory.");
            }
        }
        info!("Clearing {output_dir:?}");
        sh.remove_path(&output_dir)?;
    }

    info!("Getting crate name");
    let crate_name = get_crate_name(&sh)?;
    info!("Crate name is {crate_name}");

    info!("Generating JSON through rustdoc");
    const DEFAULT_RUSTDOC_ARGUMENTS: &str = "-Z unstable-options --output-format=json";
    let rustdoc_arguments = if let Some(rustdoc_arguments) = rustdoc_arguments {
        rustdoc_arguments + " " + DEFAULT_RUSTDOC_ARGUMENTS
    } else {
        DEFAULT_RUSTDOC_ARGUMENTS.to_owned()
    };
    println!("rustdoc arguments: \"{rustdoc_arguments}\"");
    let cargo_doc_arguments = cargo_doc_arguments.unwrap_or_default();
    cmd!(sh, "{cargo_command} doc --no-deps {cargo_doc_arguments...}")
        .env("RUSTDOCFLAGS", rustdoc_arguments)
        .env_remove("RUSTFLAGS")
        .run()?;

    let json_path = format!("./target/doc/{crate_name}.json");
    info!("Reading outputted JSON from {json_path}");
    let json = sh.read_file(&json_path)?;

    info!("Deserializing JSON");
    let json: Crate = serde_json::from_str(&json)?;

    info!("Generating markdown from JSON");
    let mut handled_ids = vec![];
    for (id, item) in json.paths.iter().filter(|(_, i)| i.crate_id == 0) {
        handle_item(
            &sh,
            &json,
            &output_dir,
            &mut handled_ids,
            id,
            json.index.get(id).expect("rustdoc JSON output is invalid?"),
            &item.path,
            &item.kind,
        )?;
    }

    Ok(())
}
