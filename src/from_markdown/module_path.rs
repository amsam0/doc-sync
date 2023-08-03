use std::fmt::{Debug, Display};
use std::ops::Deref;

/// Relative to `crate`
pub struct RelativeModulePath(Vec<String>);
impl RelativeModulePath {
    pub fn from_file_path(file_path: &str) -> RelativeModulePath {
        let mut file_module_path = vec![];
        for part in file_path.split("/") {
            if part.is_empty()
                || part == "src"
                || part == "mod.rs"
                || part == "lib.rs"
                || part == "main.rs"
            {
                continue;
            }
            if !part.ends_with(".rs") {
                file_module_path.push(part.to_owned());
            } else {
                // Remove the .rs
                file_module_path.push(part[..(part.len() - 3)].to_owned());
            }
        }
        RelativeModulePath(file_module_path)
    }
}
impl Display for RelativeModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &(if !self.0.is_empty() {
                format!("crate::{}", self.0.join("::"))
            } else {
                "crate".to_owned()
            }),
        )
    }
}
impl Debug for RelativeModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Deref for RelativeModulePath {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AbsoluteModulePath<'lt>(pub &'lt Vec<String>);
impl Display for AbsoluteModulePath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &(if !self.0.is_empty() {
                self.0.join("::")
            } else {
                "::".to_owned()
            }),
        )
    }
}
impl Debug for AbsoluteModulePath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Deref for AbsoluteModulePath<'_> {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
