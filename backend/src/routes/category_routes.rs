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
    category_service::create_category(db, new_category.into_inner())
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

        let test_category = categories.first().unwrap();
        assert_eq!(test_category.id, 1);
        assert_eq!(test_category.name, "Divertissement");
        assert!(test_category.icon.is_none());
    }

    #[test]
    fn find_category_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get(format!("/categories/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.name, "Divertissement");
        assert!(category.icon.is_none());
    }

    #[test]
    fn create_category_find_by_id_and_delete_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let new_category = CategoryDTO {
            id: -1,
            name: String::from("test_new_category"),
            icon: Option::None,
        };
        let response = client
            .post(uri!("/categories", super::create_category))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_category).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.name, "test_new_category");
        assert!(category.icon.is_none());

        let response = client
            .get(format!("/categories/{}", category.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let category = response.into_json::<CategoryDTO>().unwrap();
        assert_eq!(category.name, "test_new_category");
        assert!(category.icon.is_none());

        let response = client
            .delete(format!("/categories/{}", category.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
