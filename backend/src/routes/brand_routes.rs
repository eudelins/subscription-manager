use crate::services::brand_service;
use crate::dto_entities::brand_dto::BrandDTO;
use crate::SubscriptionsDb;

use rocket_db_pools::Connection;
use rocket::serde::json::Json;


#[get("/<id>")]
pub async fn find_brand_by_id(
    db: Connection<SubscriptionsDb>,
    id: i32
) -> Option<Json<BrandDTO>> {
    brand_service::find_brand_by_id(db, id).await.map(Json)
}

#[get("/")]
pub async fn find_all_brands(
    db: Connection<SubscriptionsDb>
) -> Json<Vec<BrandDTO>> {
    Json(brand_service::find_all_brands(db).await)
}

#[post("/", format = "application/json", data = "<new_brand>")]
pub async fn create_brand (
    db: Connection<SubscriptionsDb>,
    new_brand: Json<BrandDTO>
) -> Option<Json<BrandDTO>> {
    brand_service::create_brand(db, new_brand.into_inner()).await.map(Json)
}

#[delete("/<id>")]
pub async fn delete_brand_by_id(
    db: Connection<SubscriptionsDb>,
    id: i32
) {
    brand_service::delete_brand_by_id(db, id).await
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test::get_test_client;
    use crate::rocket;
    use rocket::http::{Status, Header};
    use rocket::serde::json::to_string;

    #[test]
    fn find_all_brands_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get(uri!("/brands", super::find_all_brands)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let brands = response.into_json::<Vec<BrandDTO>>().unwrap();
        assert_eq!(brands.len(), 1);

        let test_brand = brands.first().unwrap();
        assert_eq!(test_brand.id, 1);
        assert_eq!(test_brand.name, "Amazon");
        assert!(test_brand.logo.is_none());
    }

    #[test]
    fn find_brand_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get(format!("/brands/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.name, "Amazon");
        assert!(brand.logo.is_none());
    }

    #[test]
    fn create_brand_find_by_id_and_delete_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let new_brand = BrandDTO {
            id: -1,
            name: String::from("test_new_brand"),
            logo: Option::None
        };
        let response = client.post(uri!("/brands", super::create_brand))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_brand).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.name, "test_new_brand");
        assert!(brand.logo.is_none());
        
        let response = client.get(format!("/brands/{}", brand.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let brand = response.into_json::<BrandDTO>().unwrap();
        assert_eq!(brand.name, "test_new_brand");
        assert!(brand.logo.is_none());

        let response = client.delete(format!("/brands/{}", brand.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}