use rocket::{fs::TempFile, http::ContentType, tokio::fs::File};
use rocket_db_pools::Connection;
use std::io::{self, ErrorKind};
use std::process::Command;

use super::{brand_service, category_service};
use crate::dto_entities::upload_dto::UploadType;
use crate::SubscriptionsDb;

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
    db: Connection<SubscriptionsDb>,
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
        file.persist_to(upload_path).await?;
        let result = match upload_type {
            UploadType::Brand => {
                brand_service::update_brand_logo(db, id, Some(format!("{id}.img").as_str())).await
            }
            UploadType::Category => {
                category_service::update_category_icon(db, id, Some(format!("{id}.img").as_str()))
                    .await
            }
        };
        return result.ok_or(io::Error::new(
            ErrorKind::InvalidData,
            "Element update failed",
        ));
    }
    Err(io::Error::new(
        ErrorKind::InvalidData,
        "The file type is not accepted",
    ))
}

pub async fn delete_file(
    db: Connection<SubscriptionsDb>,
    upload_type: UploadType,
    id: i32,
) -> io::Result<()> {
    let upload_path = format!(
        "{}/uploads/{}/{}.img",
        env!("CARGO_MANIFEST_DIR"),
        upload_type.into_upload_dir(),
        id
    );
    Command::new("rm").arg(upload_path).status()?;
    let result = match upload_type {
        UploadType::Brand => brand_service::update_brand_logo(db, id, Option::None).await,
        UploadType::Category => category_service::update_category_icon(db, id, Option::None).await,
    };
    result.ok_or(io::Error::new(
        ErrorKind::InvalidData,
        "Element delete failed",
    ))
}
