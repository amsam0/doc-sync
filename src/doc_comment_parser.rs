use std::{collections::VecDeque, str::FromStr};

use pest::Parser;
use pest_derive::Parser;
use tracing::trace;

#[derive(Parser)]
#[grammar = "doc_comment.pest"]
struct DocCommentParser;

pub struct DocComment {
    pub comment_type: DocCommentType,
    // pub comment_lines: Vec<String>,
    pub start_index: usize,
    pub end_index: usize,
}

#[derive(Debug)]
pub enum DocCommentType {
    /// ///
    OuterSingle,
    /// //!
    InnerSingle,
    /// /** */
    OuterMulti,
    /// /*! */
    InnerMulti,
}

impl DocCommentType {
    pub fn edit_lines_for_comment_type(&self, lines: Vec<String>) -> Vec<String> {
        use DocCommentType::*;
        match &self {
            OuterSingle => lines
                .into_iter()
                .map(|mut l| {
                    l.insert_str(0, "/// ");
                    l
                })
                .collect(),
            InnerSingle => lines
                .into_iter()
                .map(|mut l| {
                    l.insert_str(0, "//! ");
                    l
                })
                .collect(),
            OuterMulti => {
                let mut lines = VecDeque::from(lines);
                lines.push_front("/**".to_owned());
                lines.push_back("*/".to_owned());
                lines.into()
            }
            InnerMulti => {
                let mut lines = VecDeque::from(lines);
                lines.push_front("/*!".to_owned());
                lines.push_back("*/".to_owned());
                lines.into()
            }
        }
    }
}

#[derive(Debug)]
pub struct InvalidDocCommentType(String);
impl FromStr for DocCommentType {
    type Err = InvalidDocCommentType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DocCommentType::*;
        match s {
            "///" => Ok(OuterSingle),
            "//!" => Ok(InnerSingle),
            "/**" => Ok(OuterMulti),
            "/*!" => Ok(InnerMulti),
            _ => Err(InvalidDocCommentType(s.to_owned())),
        }
    }
}

#[tracing::instrument(skip(input))]
pub fn get_doc_comment(input: &String) -> Option<DocComment> {
    let item = DocCommentParser::parse(Rule::item, &input)
        .ok()?
        .next()
        .unwrap()
        // This should give us the doc_comment itself, which we can use to get the start and end index
        .into_inner()
        .next()
        .unwrap();

    let mut comment_type = None;
    // let mut comment_lines = vec![];

    let item_span = item.as_span();
    let start_index = item_span.start();
    let end_index = item_span.end();

    // Now let's go through the doc_comment
    // This will only match children of doc_comment_singleline or doc_comment_multiline
    for rule in item.into_inner() {
        match rule.as_rule() {
            Rule::doc_comment_singleline_type | Rule::doc_comment_multiline_type => {
                comment_type = Some(
                    rule.as_str()
                        .parse()
                        .expect("Invalid comment type, please report on GitHub Issues since this should be impossible to trigger"),
                );
            }
            // Rule::doc_comment_singleline_text | Rule::doc_comment_multiline_text => {
            //     comment_lines.push(rule.as_str().to_owned());
            // }
            _ => {}
        }
    }

    let comment_type = match comment_type {
        Some(c) => c,
        None => return None,
    };

    trace!(comment_type = debug(&comment_type), start_index, end_index);

    Some(DocComment {
        comment_type,
        // comment_lines,
        start_index,
        end_index,
    })
}
