use std::collections::HashMap;

use super::subscription_service;
use crate::{
    dto_entities::{dataviz_dto::StatisticsDTO, subscription_dto::EntireSubscriptionDTO},
    SubscriptionsDb,
};

use rocket_db_pools::Connection;

pub async fn get_statistics(db: Connection<SubscriptionsDb>) -> StatisticsDTO {
    let active_subs = subscription_service::find_all_subscriptions_with_categories(db)
        .await
        .into_iter()
        .filter(|s| s.status)
        .collect::<Vec<EntireSubscriptionDTO>>();
    StatisticsDTO::new(
        get_monthly_expense(&active_subs),
        get_monthly_expense_by_category(&active_subs),
        active_subs.iter().count(),
    )
}

pub fn get_monthly_expense(active_subs: &Vec<EntireSubscriptionDTO>) -> f32 {
    active_subs.iter().map(|s| s.price).sum()
}

pub fn get_monthly_expense_by_category(
    active_subs: &Vec<EntireSubscriptionDTO>,
) -> HashMap<i32, f32> {
    let mut monthly_expenses = HashMap::<i32, f32>::new();
    active_subs.iter().for_each(|s| {
        s.categories_id.iter().for_each(|c_id| {
            monthly_expenses.insert(*c_id, s.price + monthly_expenses.get(&c_id).unwrap_or(&0.0));
        });
    });
    monthly_expenses
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use crate::utils::test::get_test_client;
    use rocket::http::Status;

    #[test]
    fn get_monthly_expense_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get("/subscriptions/with-categories").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let subs = response.into_json::<Vec<EntireSubscriptionDTO>>().unwrap();
        let active_subs = subs
            .into_iter()
            .filter(|s| s.status)
            .collect::<Vec<EntireSubscriptionDTO>>();
        let monthly_expense = get_monthly_expense(&active_subs);
        assert_eq!(monthly_expense, 20.09);
    }

    #[test]
    fn get_monthly_expense_by_category_test() {
        let client = get_test_client().lock().unwrap();
        let response = client.get("/subscriptions/with-categories").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let subs = response.into_json::<Vec<EntireSubscriptionDTO>>().unwrap();
        let active_subs = subs
            .into_iter()
            .filter(|s| s.status)
            .collect::<Vec<EntireSubscriptionDTO>>();
        let monthly_expense_by_category = get_monthly_expense_by_category(&active_subs);
        assert_eq!(monthly_expense_by_category.get(&1).unwrap(), &20.09);
        assert_eq!(monthly_expense_by_category.get(&2).unwrap(), &10.1);
    }
}
