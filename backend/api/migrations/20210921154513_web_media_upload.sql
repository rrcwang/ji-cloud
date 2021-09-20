create table web_media_upload (
    media_id uuid primary key references web_media_library(id) on delete restrict,
    uploaded_at timestamptz,
    -- if `uploaded_at is not null and processed_at >= uploaded_at is not true` at, this image hasn't been processed yet.
    processed_at timestamptz,
    -- null if not processed, `is true` if the uploaded was successful, `is not true` otherwise.
    processing_result boolean
);