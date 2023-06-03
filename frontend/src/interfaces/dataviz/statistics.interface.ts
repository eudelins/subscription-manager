interface Statistics {
  monthlyExpense: number;
  monthlyExpenseByCategory: Record<number, string>;
  numberOfSubcriptions: number;
}

export default Statistics;
