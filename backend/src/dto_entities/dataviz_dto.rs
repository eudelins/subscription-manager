use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StatisticsDTO {
    pub monthly_expense: f32,
    pub monthly_expense_by_category: HashMap<i32, f32>,
    pub number_of_subscriptions: usize,
}

impl StatisticsDTO {
    pub fn new(
        monthly_expense: f32,
        monthly_expense_by_category: HashMap<i32, f32>,
        number_of_subscriptions: usize,
    ) -> Self {
        StatisticsDTO {
            monthly_expense,
            monthly_expense_by_category,
            number_of_subscriptions,
        }
    }
}
