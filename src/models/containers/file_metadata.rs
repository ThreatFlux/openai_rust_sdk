//! Current container-file metadata, separate from the legacy helper models.

use crate::{De, Ser};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Metadata returned by the official container-file endpoints.
#[derive(Debug, Clone, Ser, De)]
pub struct ContainerFileMetadata {
    /// File identifier.
    pub id: String,
    /// Size in bytes.
    pub bytes: u64,
    /// Owning container identifier.
    pub container_id: String,
    /// Creation time as a Unix timestamp.
    pub created_at: u64,
    /// Resource discriminator (`container.file`).
    pub object: String,
    /// Path inside the container.
    pub path: String,
    /// Origin, such as `user` or `assistant`; future origins remain representable.
    pub source: String,
    /// Additional server metadata.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A cursor-paginated page of current container-file metadata.
#[derive(Debug, Clone, Ser, De)]
pub struct ContainerFileMetadataList {
    /// Resource discriminator (`list`).
    pub object: String,
    /// Files on this page.
    pub data: Vec<ContainerFileMetadata>,
    /// First identifier, when present.
    pub first_id: Option<String>,
    /// Last identifier, when present.
    pub last_id: Option<String>,
    /// Whether a subsequent page exists.
    pub has_more: bool,
    /// Additional page metadata.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
