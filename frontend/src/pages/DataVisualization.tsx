import { CSSProperties, useEffect, useState } from 'react';

import { Card, Space } from 'antd';
import { Bar, BarChart, CartesianGrid, Tooltip, XAxis, YAxis } from 'recharts';

import { getStatistics } from 'services/dataviz';
import Statistics from 'interfaces/dataviz/statistics.interface';
import Category from 'interfaces/categories/category.interface';
import { getAllCategories } from 'services/categories';

function DataVisualization() {
  const [statistics, setStatistics] = useState<Statistics>();
  const [categories, setCategories] = useState<Category[]>([]);
  const [monthlyExpenseByCategory, setMonthlyExpenseByCategory] = useState<any[]>([]);

  useEffect(() => {
    Promise.all([getStatistics(), getAllCategories()]).then(([stats, cats]) => {
      setStatistics(stats);
      setCategories(cats);
      setMonthlyExpenseByCategory(
        Object.entries(stats.monthlyExpenseByCategory).map(([id, value]) => {
          const catName = cats.find((cat) => cat.id === parseInt(id))?.name;
          return { name: catName, value };
        })
      );
    });
  }, []);

  return (
    <>
      <Space size={150} style={{ marginLeft: 150, marginBottom: 80, textAlign: 'center' }}>
        <Card style={cardStyle}>
          <p>Frais mensuel</p>
          <p>{statistics?.monthlyExpense} €</p>
        </Card>
        <Card style={cardStyle}>
          <p>Nombre d'abonnements en cours</p>
          <p>{statistics?.numberOfSubcriptions}</p>
        </Card>
      </Space>
      <BarChart margin={{ left: 200 }} width={1150} height={250} data={monthlyExpenseByCategory}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis dataKey="name" />
        <YAxis />
        <Tooltip />
        <Bar dataKey="value" fill="#82ca9d" />
      </BarChart>
    </>
  );
}

const cardStyle: CSSProperties = { width: 500, fontSize: 24, borderColor: 'black' };

export default DataVisualization;
