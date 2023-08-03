mod code_blocks;

pub fn from_markdown(lines: &mut Vec<String>) {
    code_blocks::from_markdown(lines);
}

pub fn to_markdown(lines: &mut Vec<String>) {
    code_blocks::to_markdown(lines);
}
