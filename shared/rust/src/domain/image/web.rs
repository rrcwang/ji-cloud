//! Types for web image library.

use serde::{Deserialize, Serialize};

use super::{ImageId, ImageKind};

/// Request for creating a user image profile
#[derive(Serialize, Deserialize, Debug)]
pub struct WebImageCreateRequest {
    /// The kind of the image. Most relevant for uploading user profile images
    pub kind: ImageKind,
}
