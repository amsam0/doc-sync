use std::{error::Error, path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use toml::Table;
use xshell::Shell;

mod doc_comment_parser;
mod edit_docs;
mod from_markdown;
mod to_markdown;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    ToMarkdown(ToMarkdown),
    FromMarkdown(FromMarkdown),
}

#[derive(Parser)]
/// Generates markdown files from rustdoc's JSON output.
pub struct ToMarkdown {
    #[arg(short, long, default_value = "cargo +nightly")]
    /// The cargo command that will be used to invoke `cargo doc`. Must be a nightly compiler.
    cargo_command: String,
    #[arg(short = 'd', long)]
    /// Extra cargo doc arguments.
    cargo_doc_arguments: Option<Vec<String>>,
    #[arg(short, long)]
    /// Extra rustdoc arguments.
    rustdoc_arguments: Option<String>,
    #[arg(short, long, default_value = "./target/doc-sync")]
    /// The directory to output the markdown files.
    output_dir: PathBuf,
    #[arg(short, long, default_value_t = false)]
    /// If true, doc-sync will overwrite generated markdown files even if they haven't been converted back to doc comments.
    force: bool,
}

#[derive(Parser)]
/// Updates the inline documentation using the markdown files previously generated.
pub struct FromMarkdown {
    #[arg(short, long, default_value = "./target/doc-sync")]
    /// The directory to use as input. This should be the same as the directory used when generating the markdown files.
    input_dir: PathBuf,
    #[arg(short, long, default_value_t = false)]
    /// If true, doc-sync will not exit if there are uncommitted changes.
    allow_dirty: bool,
}

fn main() {
    let args = Cli::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace")
    }
    tracing_subscriber::fmt::init();

    let sh = Shell::new().unwrap();

    match args.command {
        CliCommand::ToMarkdown(args) => to_markdown::to_markdown(sh, args),
        CliCommand::FromMarkdown(args) => from_markdown::from_markdown(sh, args),
    }
}

mod consts {
    pub const METADATA_COMMENT_PREFIX: &str =
        "<!-- DO NOT REMOVE OR EDIT THIS LINE! Otherwise, doc-sync will break! ";
    pub const METADATA_COMMENT_SUFFIX: &str = " -->";
    pub const METADATA_ID_PREFIX: &str = "DOC_SYNC_RUSTDOC_ID=\"";
    pub const METADATA_ID_SUFFIX: &str = "\"";
}

#[wrap_match::wrap_match(log_success = false)]
fn get_crate_name(sh: &Shell) -> Result<String, Box<dyn Error>> {
    let manifest = sh.read_file("./Cargo.toml")?;
    let manifest: Table = Table::from_str(&manifest)?;
    Ok(manifest["package"]["name"]
        .as_str()
        .expect("name is not a string")
        .to_owned()
        // rustdoc will use the "real" crate name when it crates the output json. the "real" crate name will have dashes replaced with underscores
        .replace("-", "_"))
}
