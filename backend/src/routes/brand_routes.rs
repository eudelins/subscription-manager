use crate::dto_entities::brand_dto::BrandDTO;
use crate::services::brand_service;
use crate::SubscriptionsDb;

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/<id>")]
pub async fn find_brand_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<Json<BrandDTO>> {
    brand_service::find_brand_by_id(db, id).await.map(Json)
}

#[get("/")]
pub async fn find_all_brands(db: Connection<SubscriptionsDb>) -> Json<Vec<BrandDTO>> {
    Json(brand_service::find_all_brands(db).await)
}

#[post("/", format = "application/json", data = "<new_brand>")]
pub async fn create_brand(
    db: Connection<SubscriptionsDb>,
    new_brand: Json<BrandDTO>,
) -> Option<Json<BrandDTO>> {
    brand_service::create_or_update_brand(db, new_brand.into_inner(), false)
        .await
        .map(Json)
}

#[put("/", format = "application/json", data = "<new_brand>")]
pub async fn update_brand(
    db: Connection<SubscriptionsDb>,
    new_brand: Json<BrandDTO>,
) -> Option<Json<BrandDTO>> {
    brand_service::create_or_update_brand(db, new_brand.into_inner(), true)
        .await
        .map(Json)
}

#[delete("/<id>")]
pub async fn delete_brand_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    brand_service::delete_brand_by_id(db, id).await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use crate::utils::test::get_test_client;
    use rocket::http::{Header, Status};
    use rocket::serde::json::to_string;

    #[test]
    fn find_all_brands_test() {
        let client = get_test_client().lock().unwrap();
        let response = client
            .get(uri!("/brands", super::find_all_brands))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let brands = response.into_json::<Vec<BrandDTO>>().unwrap();
        assert_eq!(brands.len(), 3);

        let test_brand = brands.iter().find(|s| s.id == 1).unwrap();
        assert_eq!(test_brand.id, 1);
        assert_eq!(test_brand.name, "Amazon");
    }

    #[test]
    fn find_brand_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get(format!("/brands/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.name, "Amazon");
    }

    #[test]
    fn create_brand_find_by_id_and_delete_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let new_brand = BrandDTO {
            id: -1,
            name: String::from("test_new_brand"),
        };
        let response = client
            .post(uri!("/brands", super::create_brand))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_brand).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.name, "test_new_brand");

        let response = client.get(format!("/brands/{}", brand.id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.name, "test_new_brand");

        let response = client.delete(format!("/brands/{}", brand.id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn update_brand_twice_and_find_by_id_test() {
        let client = get_test_client().lock().unwrap();

        // Update
        let new_brand = BrandDTO {
            id: 1,
            name: String::from("test_update"),
        };
        let response = client
            .put(uri!("/brands", super::update_brand))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_brand).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.name, "test_update");

        // Check with fetch
        let response = client.get(format!("/brands/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.id, 1);
        assert_eq!(brand.name, "test_update");

        // Go back to as it was
        let original_sub = BrandDTO {
            id: 1,
            name: String::from("Amazon"),
        };
        let response = client
            .put(uri!("/brands", super::update_brand))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&original_sub).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Check with fetch
        let response = client.get(format!("/brands/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.id, 1);
        assert_eq!(brand.name, "Amazon");
    }
}
