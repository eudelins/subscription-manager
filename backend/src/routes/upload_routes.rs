use crate::SubscriptionsDb;
use crate::{dto_entities::upload_dto::UploadType, services::upload_service};

use rocket::{form::Form, fs::TempFile, tokio::fs::File};
use rocket_db_pools::Connection;
use std::{io, str::FromStr};

#[get("/<upload_dir>/<id>")]
pub async fn retrieve(upload_dir: &str, id: i32) -> io::Result<File> {
    let upload_type = UploadType::from_str(upload_dir)?;
    upload_service::retrieve(upload_type, id).await
}

#[derive(FromForm)]
pub struct FileUpload<'r> {
    file: TempFile<'r>,
}

#[post("/<upload_dir>/<id>", data = "<upload_req>")]
pub async fn upload(
    db: Connection<SubscriptionsDb>,
    id: i32,
    upload_dir: &str,
    mut upload_req: Form<FileUpload<'_>>,
) -> io::Result<()> {
    let upload_type = UploadType::from_str(upload_dir)?;
    upload_service::upload_file(db, &mut upload_req.file, upload_type, id).await
}

#[delete("/<upload_dir>/<id>")]
pub async fn delete_file(
    db: Connection<SubscriptionsDb>,
    id: i32,
    upload_dir: &str,
) -> io::Result<()> {
    let upload_type = UploadType::from_str(upload_dir)?;
    upload_service::delete_file(db, upload_type, id).await
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use crate::utils::test::get_test_client;
    use rocket::http::{Header, Status};
    use sha256::digest;
    use std::{fs::File, io::Read, process::Command};

    #[test]
    fn retrieve_brand_logo_test() {
        let status = Command::new("cp")
            .arg(format!("{}/uploads/tauri.png", env!("CARGO_MANIFEST_DIR")))
            .arg(format!(
                "{}/uploads/test/brands/1.img",
                env!("CARGO_MANIFEST_DIR")
            ))
            .status()
            .expect("failed to execute command");
        assert!(status.success());
        let client = get_test_client().lock().unwrap();
        let response = client.get("/uploads/brands/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            "6fc3751247d4c256e846f09445583125e692df5a28aa51027d6ff5429ced4efc",
            digest(response.into_bytes().unwrap().as_slice())
        );

        let status = Command::new("rm")
            .arg(format!(
                "{}/uploads/test/brands/1.img",
                env!("CARGO_MANIFEST_DIR")
            ))
            .status()
            .expect("failed to execute command");
        assert!(status.success());
    }

    #[test]
    fn upload_then_delete_category_icon_test() {
        let client = get_test_client().lock().unwrap();

        let boundary = "324675067314323631743614316606";
        let body = format!(
            "-----------------------------{}\r\n{}\r\n{}\r\n\r\n",
            boundary,
            "Content-Disposition: form-data; name=\"file\"; filename=\"tauri.png\"",
            "Content-Type: image/png",
        );
        let mut body_buffer = Vec::<u8>::from(body.as_bytes());
        let res = File::open(format!("{}/uploads/tauri.png", env!("CARGO_MANIFEST_DIR")))
            .unwrap()
            .read_to_end(&mut body_buffer);
        assert!(res.is_ok());
        body_buffer.append(&mut Vec::<u8>::from(format!(
            "\r\n-----------------------------{}--\r\n",
            boundary
        )));

        let response = client
            .post("/uploads/categories/1")
            .header(Header::new(
                "Content-type",
                format!(
                    "multipart/form-data; boundary=---------------------------{}",
                    boundary
                ),
            ))
            .body(body_buffer)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        assert_eq!(
            "6fc3751247d4c256e846f09445583125e692df5a28aa51027d6ff5429ced4efc",
            sha256::digest(
                std::fs::read(format!(
                    "{}/uploads/test/categories/1.img",
                    env!("CARGO_MANIFEST_DIR")
                ))
                .unwrap()
                .as_slice()
            )
        );

        let response = client.delete("/uploads/categories/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
