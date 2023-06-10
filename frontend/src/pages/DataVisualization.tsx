import { useEffect, useState } from 'react';

import { getStatistics } from 'services/dataviz';
import Statistics from 'interfaces/dataviz/statistics.interface';

import { Card, Space } from 'antd';

function DataVisualization() {
  const [statistics, setStatistics] = useState<Statistics>();

  useEffect(() => {
    getStatistics().then((res) => setStatistics(res));
  }, []);

  return (
    <>
      <Space size="large">
        <Card style={{ width: 300, fontSize: 24 }}>
          <p>{statistics?.monthlyExpense} €</p>
          <p>de frais mensuel</p>
        </Card>
        <Card style={{ width: 300, fontSize: 24 }}>
          <p>{statistics?.numberOfSubcriptions}</p>
          <p>Abonnements en cours</p>
        </Card>
      </Space>
      <p>{JSON.stringify(statistics?.monthlyExpenseByCategory)}</p>
    </>
  );
}

export default DataVisualization;
