use proc_macro2::{LineColumn, Span};
use tracing::debug;

use crate::doc_comment_parser::{get_doc_comment, DocCommentType};

#[tracing::instrument(skip(file_string))]
pub fn update_docs_for_span(
    span: Span,
    new_docs: Vec<String>,
    file_string: &mut String,
    default_comment_type: DocCommentType,
    add_extra_newline_for_new_comments: bool,
) {
    let span_start_pos = span.start().byte_pos(file_string);
    let span_source_text = span.source_text().unwrap();

    match get_doc_comment(&span_source_text) {
        Some(doc_comment) => {
            let docs = doc_comment
                .comment_type
                .edit_lines_for_comment_type(new_docs)
                .join("\n");
            let range = (span_start_pos + doc_comment.start_index)
                ..(span_start_pos + doc_comment.end_index);

            debug!(docs, range = debug(&range), "Replacing");
            file_string.replace_range(range, &docs);
        }
        None => {
            let LineColumn { line, column } = span.start();
            debug!(line, column);
            insert_new_doc_comment(
                span_start_pos,
                new_docs,
                file_string,
                default_comment_type,
                add_extra_newline_for_new_comments,
            );
        }
    }
}

#[tracing::instrument(skip(file_string))]
pub fn insert_new_doc_comment(
    index: usize,
    new_docs: Vec<String>,
    file_string: &mut String,
    default_comment_type: DocCommentType,
    add_extra_newline_for_new_comments: bool,
) {
    let mut docs = default_comment_type
        .edit_lines_for_comment_type(new_docs)
        .join("\n");
    docs.push_str("\n");
    if add_extra_newline_for_new_comments {
        docs.push_str("\n");
    }

    debug!(docs, "Inserting");
    file_string.insert_str(index, &docs);
}

#[easy_ext::ext]
impl LineColumn {
    fn byte_pos(self, file: &String) -> usize {
        let mut i = self.column;
        if self.line > 1 {
            let final_line = self.line - 1; // Make it 0 indexed
            for (line_num, line) in file.split("\n").enumerate() {
                if line_num >= final_line {
                    break;
                }
                const NEWLINE_LENGTH: usize = "\n".len();
                i += line.len() + NEWLINE_LENGTH;
            }
        }
        i
    }
}
