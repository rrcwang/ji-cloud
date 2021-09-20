use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        image::{
            user::{
                WebImageCreateRequest,
                WebImageResponse, WebImageUploadRequest, WebImageUploadResponse,
            },
            ImageId,
        },
        CreateResponse,
    },
    error::EmptyError,
};

/// Get an web library image by ID.
///
/// # Errors
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image with the requested ID is not found for the user.
/// Note that it will still return NOT_FOUND if an web image with the ID exists but is not owner by the
/// requesting user.
/// * TODO other errors here...
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = WebImageResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/web/{id}";
    const METHOD: Method = Method::Get;
}

/// Create a web library image.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = WebImageCreateRequest;
    type Res = CreateResponse<ImageId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/web";
    const METHOD: Method = Method::Post;
}

/// Upload an image to the web image library.
/// # Flow:
///
/// 1. User requests an upload session URI directly to Google Cloud Storage
///     a. User uploads to processing bucket
/// 2. Firestore is notified of `processing = true, ready = false` status at document `uploads/media/user/{id}`
/// 3. Animation is processed and uploaded to the final bucket
/// 4. Firestore is notified of `processing = true, ready = true` status at document `uploads/media/user/{id}`
///
/// # Notes:
///
/// * Can be used to update the raw data associated with the image.
/// * If the client wants to re-upload an image after it has been successfully processed, it must repeat
/// the entire flow instead of uploading to the same session URI.
///
/// # Errors:
///
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`501 - NotImplemented`](http::StatusCode::NOT_IMPLEMENTED) when the s3/gcs service is disabled.
pub struct Upload;
impl ApiEndpoint for Upload {
    // raw bytes
    type Req = WebImageUploadRequest;
    type Res = WebImageUploadResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/web/{id}/raw";
    const METHOD: Method = Method::Put;
}
