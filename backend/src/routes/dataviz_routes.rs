use crate::dto_entities::dataviz_dto::StatisticsDTO;
use crate::services::dataviz_service;
use crate::SubscriptionsDb;

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/statistics")]
pub async fn get_statistics(db: Connection<SubscriptionsDb>) -> Json<StatisticsDTO> {
    Json(dataviz_service::get_statistics(db).await)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use crate::utils::test::get_test_client;
    use rocket::http::Status;

    #[test]
    fn get_statistics_test() {
        let client = get_test_client().lock().unwrap();
        let response = client
            .get(uri!("/dataviz/", super::get_statistics))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let statistics = response.into_json::<StatisticsDTO>().unwrap();
        assert_eq!(statistics.monthly_expense, 20.09);
        assert_eq!(
            statistics.monthly_expense_by_category.get(&1).unwrap(),
            &20.09
        );
        assert_eq!(
            statistics.monthly_expense_by_category.get(&2).unwrap(),
            &10.1
        );
    }
}
