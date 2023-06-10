import axios from 'axios';

import Statistics from 'interfaces/dataviz/statistics.interface';

import { DATAVIZ_API_PATH } from './utils/path';

export async function getStatistics(): Promise<Statistics> {
  const response = await axios.get(DATAVIZ_API_PATH + 'statistics');
  return {
    monthlyExpense: response.data.monthly_expense,
    monthlyExpenseByCategory: response.data.monthly_expense_by_category,
    numberOfSubcriptions: response.data.number_of_subscriptions
  };
}
