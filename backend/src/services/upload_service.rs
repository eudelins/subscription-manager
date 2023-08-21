use rocket::{fs::TempFile, http::ContentType, tokio::fs::File};
use std::io::{self, ErrorKind};
use std::process::Command;

use crate::dto_entities::upload_dto::UploadType;

pub async fn retrieve(upload_type: UploadType, id: i32) -> io::Result<File> {
    let file_path = format!(
        "{}/uploads/{}/{}.img",
        env!("CARGO_MANIFEST_DIR"),
        upload_type.into_upload_dir(),
        id
    );
    File::open(&file_path).await
}

const ALLOWED_CONTENT_TYPES: [ContentType; 4] = [
    ContentType::PNG,
    ContentType::JPEG,
    ContentType::SVG,
    ContentType::Icon,
];

pub async fn upload_file(
    file: &mut TempFile<'_>,
    upload_type: UploadType,
    id: i32,
) -> io::Result<()> {
    if ALLOWED_CONTENT_TYPES.contains(file.content_type().unwrap_or(&ContentType::Plain)) {
        let upload_path = format!(
            "{}/uploads/{}/{}.img",
            env!("CARGO_MANIFEST_DIR"),
            upload_type.into_upload_dir(),
            id
        );
        return file.persist_to(upload_path).await.map(|_| ());
    }
    Err(io::Error::new(
        ErrorKind::InvalidData,
        "The file type is not accepted",
    ))
}

pub async fn delete_file(upload_type: UploadType, id: i32) -> io::Result<()> {
    let upload_path = format!(
        "{}/uploads/{}/{}.img",
        env!("CARGO_MANIFEST_DIR"),
        upload_type.into_upload_dir(),
        id
    );
    Command::new("rm").arg(upload_path).status().map(|_| ())
}
