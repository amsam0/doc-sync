use std::error::Error;

use syn::spanned::Spanned;
use tracing::{debug, trace};
use xshell::Shell;

use crate::doc_comment_parser::DocCommentType;

use super::item_info::ItemInfo;

mod resolve_item;
use self::resolve_item::resolve_item;

mod update_docs_for_span;
use self::update_docs_for_span::{insert_new_doc_comment, update_docs_for_span};

#[wrap_match::wrap_match(disregard_result = true)]
#[tracing::instrument(skip(sh))]
pub fn update_docs(sh: &Shell, item: ItemInfo) -> Result<(), Box<dyn Error>> {
    debug!(new_docs = item.new_docs.join("\n"));

    let mut file_contents = sh.read_file(&item.file_path)?;
    let syn_file = syn::parse_file(&file_contents)?;
    if item.item_path.is_empty() {
        // The item is the file itself

        // If we simply use the span of the file,
        // the doc comment parser will incorrectly
        // find a doc comment to replace if there
        // are no file wide doc comments.

        if !syn_file.attrs.is_empty() {
            // To avoid this, we can combine the spans
            // of all attributes to get the top of the
            // file (the part we want)A

            trace!(file_attrs_len = syn_file.attrs.len());

            let mut span = syn_file.attrs[0].span();
            if syn_file.attrs.len() > 1 {
                for attr in &syn_file.attrs[1..] {
                    span = span.join(attr.span()).unwrap(); // Should be safe since the attributes are guaranteed to be in the same file
                }
            }

            update_docs_for_span(
                span,
                item.new_docs,
                &mut file_contents,
                DocCommentType::InnerSingle,
                true,
            );
        } else {
            // However, if there are no attributes,
            // we just insert the doc comment at the top

            insert_new_doc_comment(
                0,
                item.new_docs,
                &mut file_contents,
                DocCommentType::InnerSingle,
                true,
            );
        }
    } else {
        // We need to resolve the item in the file

        let syn_item = resolve_item(&syn_file.items, item.item_path)?;
        update_docs_for_span(
            syn_item.inner().span(),
            item.new_docs,
            &mut file_contents,
            DocCommentType::OuterSingle,
            false,
        );
    }

    sh.write_file(format!("target/{}", item.file_path), file_contents)?;

    Ok(())
}
