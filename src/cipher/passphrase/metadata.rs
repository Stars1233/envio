use super::v1::MetadataV1;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "version")]
pub enum VersionedMetadata {
    #[serde(rename = "1")]
    V1(MetadataV1),
}

impl Default for VersionedMetadata {
    fn default() -> Self {
        VersionedMetadata::V1(Default::default())
    }
}

impl From<MetadataV1> for VersionedMetadata {
    fn from(meta: MetadataV1) -> Self {
        VersionedMetadata::V1(meta)
    }
}
