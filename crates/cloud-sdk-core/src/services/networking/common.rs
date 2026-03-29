use serde::{Deserialize, Serialize};

/// Reference to a sub-resource by ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubResourceRef {
    pub id: String,
}
