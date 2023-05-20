use crate::services::subscription_service;
use crate::dto_entities::subscription_dto::{SubscriptionDTO, CreateOrUpdateSubscriptionDTO, EntireSubscriptionDTO};
use crate::SubscriptionsDb;

use rocket_db_pools::Connection;
use rocket::serde::json::Json;


#[get("/<id>")]
pub async fn find_subscription_by_id(
    db: Connection<SubscriptionsDb>,
    id: i32
) -> Option<Json<EntireSubscriptionDTO>> {
    subscription_service::find_subscription_by_id(db, id).await.map(Json)
}

#[get("/")]
pub async fn find_all_subscriptions(
    db: Connection<SubscriptionsDb>
) -> Json<Vec<SubscriptionDTO>> {
    Json(subscription_service::find_all_subscriptions(db).await)
}

#[post("/", format = "application/json", data = "<new_sub>")]
pub async fn create_subscription(
    db: Connection<SubscriptionsDb>,
    new_sub: Json<CreateOrUpdateSubscriptionDTO>
) -> Option<Json<SubscriptionDTO>> {
    subscription_service::create_or_update_subscription(
        db,
        new_sub.into_inner(),
        false
    ).await.map(Json)
}

#[put("/", format = "application/json", data = "<updated_sub>")]
pub async fn update_subscription(
    db: Connection<SubscriptionsDb>,
    updated_sub: Json<CreateOrUpdateSubscriptionDTO>
) -> Option<Json<SubscriptionDTO>> {
    subscription_service::create_or_update_subscription(
        db,
        updated_sub.into_inner(),
        true
    ).await.map(Json)
}

#[delete("/<id>")]
pub async fn delete_subscription_by_id(
    db: Connection<SubscriptionsDb>,
    id: i32
) -> Option<()> {
    subscription_service::delete_subscription_by_id(db, id).await
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test::get_test_client;
    use crate::rocket;
    use rocket::http::{Status, Header};
    use rocket::serde::json::to_string;

    #[test]
    fn find_all_subscriptions_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get(uri!("/subscriptions", super::find_all_subscriptions)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let subscriptions = response.into_json::<Vec<SubscriptionDTO>>().unwrap();
        assert_eq!(subscriptions.len(), 1);

        let test_subscription = subscriptions.first().unwrap();
        assert_eq!(test_subscription.id, 1);
        assert_eq!(test_subscription.name, "Prime Video");
        assert_eq!(test_subscription.price, 10.1);
        assert_eq!(test_subscription.status, false);
    }
    
    #[test]
    fn find_subscription_by_id_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get(format!("/subscriptions/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        let subscription = response.into_json::<EntireSubscriptionDTO>().unwrap();
        assert_eq!(subscription.id, 1);
        assert_eq!(subscription.name, "Prime Video");
        assert_eq!(subscription.price, 10.1);
        assert_eq!(subscription.status, false);
        assert_eq!(subscription.categories_id, vec![1, 2]);
    }
    
    #[test]
    fn create_subscription_find_by_id_and_delete_by_id_test() {
        let client = get_test_client().lock().unwrap();

        // Create new sub
        let new_subscription = CreateOrUpdateSubscriptionDTO {
            id: Option::None,
            name: String::from("test_new_subscription"),
            brand_id: 1,
            price: 7.99,
            status: true,
            categories_id: Vec::new()
        };
        let response = client.post(uri!("/subscriptions", super::create_subscription))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_subscription).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let subscription = response.into_json::<SubscriptionDTO>().unwrap();
        assert_eq!(subscription.name, "test_new_subscription");
        assert_eq!(subscription.price, 7.99);
        assert_eq!(subscription.status, true);

        // Check with fetch
        let response = client.get(format!("/subscriptions/{}", subscription.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let subscription = response.into_json::<SubscriptionDTO>().unwrap();
        assert_eq!(subscription.name, "test_new_subscription");
        assert_eq!(subscription.price, 7.99);
        assert_eq!(subscription.status, true);

        // Clean by delete
        let response = client.delete(format!("/subscriptions/{}", subscription.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn update_subscription_twice_and_find_by_id_test() {
        let client = get_test_client().lock().unwrap();

        // Update
        let new_subscription = CreateOrUpdateSubscriptionDTO {
            id: Some(1),
            name: String::from("test_update"),
            brand_id: 1,
            price: 12.99,
            status: true,
            categories_id: vec![1]
        };
        let response = client.put(uri!("/subscriptions", super::update_subscription))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&new_subscription).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let subscription = response.into_json::<SubscriptionDTO>().unwrap();
        assert_eq!(subscription.name, "test_update");
        assert_eq!(subscription.price, 12.99);
        assert_eq!(subscription.status, true);
        
        // Check with fetch
        let response = client.get(format!("/subscriptions/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let subscription = response.into_json::<EntireSubscriptionDTO>().unwrap();
        assert_eq!(subscription.id, 1);
        assert_eq!(subscription.name, "test_update");
        assert_eq!(subscription.price, 12.99);
        assert_eq!(subscription.status, true);
        assert_eq!(subscription.categories_id, vec![1]);

        // Go back to as it was
        let original_sub = CreateOrUpdateSubscriptionDTO {
            id: Some(1),
            name: String::from("Prime Video"),
            brand_id: 1,
            price: 10.1,
            status: false,
            categories_id: vec![1, 2]
        };
        let response = client.put(uri!("/subscriptions", super::update_subscription))
            .header(Header::new("Content-type", "application/json"))
            .body(to_string(&original_sub).expect("Deserealization failed"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Check with fetch
        let response = client.get(format!("/subscriptions/{}", 1)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let subscription = response.into_json::<EntireSubscriptionDTO>().unwrap();
        assert_eq!(subscription.id, 1);
        assert_eq!(subscription.name, "Prime Video");
        assert_eq!(subscription.price, 10.1);
        assert_eq!(subscription.status, false);
        assert_eq!(subscription.categories_id, vec![1, 2]);
    }
}