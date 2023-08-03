use std::collections::HashMap;
use std::ops::Deref;

use rustdoc_types::{Crate, ItemKind};
use tracing::{error, trace};

use super::module_path::{AbsoluteModulePath, RelativeModulePath};

/// Represents an item in a file
pub struct ItemPath(HashMap<String, ItemKind>);

impl ItemPath {
    #[tracing::instrument(skip(json))]
    #[inline]
    pub fn new(
        file_module_path: &RelativeModulePath,
        full_item_path: AbsoluteModulePath,
        json: &Crate,
    ) -> Option<ItemPath> {
        let mut candidates = json
            .paths
            .values()
            .filter(|i| i.crate_id == 0)
            // Exclude crate name, we already know it's the correct crate and the crate name isn't included in relative module paths
            .filter(|i| i.path[1..].starts_with(&file_module_path));

        let mut inner = HashMap::new();
        let mut current_parts = vec![];
        let offset = 1 + file_module_path.len();
        for part in &full_item_path[offset..] {
            current_parts.push(part.to_owned());
            let item = candidates
                .find(|i| &i.path[offset..] == &current_parts)
                .or_else(|| {
                    error!(current_parts = debug(&current_parts), "Couldn't find item");
                    None
                })?;
            inner.insert(part.to_owned(), item.kind.clone());
        }
        trace!(inner = debug(&inner));
        Some(ItemPath(inner))
    }
}

impl std::fmt::Debug for ItemPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ItemPath {
    type Target = HashMap<String, ItemKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
