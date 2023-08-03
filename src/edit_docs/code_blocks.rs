//! Converts ` ``` ` code blocks to ` ```rs ` when converting to markdown, and ` ```rs ` to ` ``` ` when converting from markdown.

const CODE_BLOCK: &str = "```";
const COMMON_RUST_CODE_BLOCK: &str = "```rs";

#[inline(always)]
pub fn from_markdown(lines: &mut Vec<String>) {
    for line in lines {
        *line = line
            .replace(COMMON_RUST_CODE_BLOCK, CODE_BLOCK)
            .replace("```rust", CODE_BLOCK);
    }
}

#[inline(always)]
pub fn to_markdown(lines: &mut Vec<String>) {
    let mut in_code_block = false;
    for line in lines {
        if line.trim() == CODE_BLOCK {
            if !in_code_block {
                in_code_block = true;
                // We want to keep whitespace, if there is any
                *line = line.replace(CODE_BLOCK, COMMON_RUST_CODE_BLOCK);
            } else {
                in_code_block = false;
            }
        }
    }
}
