use crate::{dto_entities::category_dto::CategoryDTO, services::category_service, SubscriptionsDb};

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/<id>")]
pub async fn find_category_by_id(
    db: Connection<SubscriptionsDb>,
    id: i32,
) -> Option<Json<CategoryDTO>> {
    category_service::find_category_by_id(db, id)
        .await
        .map(Json)
}

#[get("/")]
pub async fn find_all_categories(db: Connection<SubscriptionsDb>) -> Json<Vec<CategoryDTO>> {
    Json(category_service::find_all_categories(db).await)
}

#[post("/", format = "application/json", data = "<new_category>")]
pub async fn create_category(
    db: Connection<SubscriptionsDb>,
    new_category: Json<CategoryDTO>,
) -> Option<Json<CategoryDTO>> {
    category_service::create_or_update_category(db, new_category.into_inner(), false)
        .await
        .map(Json)
}

#[put("/", format = "application/json", data = "<new_category>")]
pub async fn update_category(
    db: Connection<SubscriptionsDb>,
    new_category: Json<CategoryDTO>,
) -> Option<Json<CategoryDTO>> {
    category_service::create_or_update_category(db, new_category.into_inner(), true)
        .await
        .map(Json)
}

#[delete("/<id>")]
pub async fn delete_category_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    category_service::delete_category_by_id(db, id).await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use crate::utils::test::get_test_client;
    use rocket::http::{Header, Status};
    use rocket::serde::json::to_string;

    #[test]
    fn find_all_categories_test() {
        let client = get_test_client().lock().unwrap();
        let response = client
            .get(uri!("/categories", super::find_all_categories))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let categories = response.into_json::<Vec<CategoryDTO>>().unwrap();
        assert_eq!(categories.len(), 2);

        let test_category = categories.iter().find(|s| s.id == 1).unwrap();
        assert_eq!(test_category.id, 1);
        assert_eq!(test_category.name, "Divertissement");
    }

    #[test]
    fn find_category_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get(format!("/categories/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.name, "Divertissement");
    }

    #[test]
    fn create_category_find_by_id_and_delete_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let new_category = CategoryDTO {
            id: -1,
            name: String::from("test_new_category"),
        };
        let response = client
            .post(uri!("/categories", super::create_category))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_category).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.name, "test_new_category");

        let response = client
            .get(format!("/categories/{}", category.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.name, "test_new_category");

        let response = client
            .delete(format!("/categories/{}", category.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn update_category_twice_and_find_by_id_test() {
        let client = get_test_client().lock().unwrap();

        // Update
        let new_category = CategoryDTO {
            id: 1,
            name: String::from("test_update"),
        };
        let response = client
            .put(uri!("/categories", super::update_category))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_category).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.name, "test_update");

        // Check with fetch
        let response = client.get(format!("/categories/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.id, 1);
        assert_eq!(category.name, "test_update");

        // Go back to as it was
        let original_sub = CategoryDTO {
            id: 1,
            name: String::from("Divertissement"),
        };
        let response = client
            .put(uri!("/categories", super::update_category))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&original_sub).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Check with fetch
        let response = client.get(format!("/categories/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.id, 1);
        assert_eq!(category.name, "Divertissement");
    }
}
