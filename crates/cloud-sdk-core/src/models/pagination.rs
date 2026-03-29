use serde::{Deserialize, Serialize};

/// Paginated list response matching Azure's `{ "value": [...], "nextLink": "..." }` pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub value: Vec<T>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            value: items,
            next_link: None,
        }
    }

    pub fn has_next(&self) -> bool {
        self.next_link.is_some()
    }
}
