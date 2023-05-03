import { useState, useEffect } from 'react';

import { getAllSubscriptions } from './services/subscriptions';
import Subscriptions from './interfaces/subscriptions/subscription.interface';

function App() {
  const [subscriptions, setSubscriptions] = useState<Subscriptions[]>([]);
  useEffect(() => {
    getAllSubscriptions().then((res) => setSubscriptions(res));
  }, []);

  return (
    <div className="container">
      <h1>Welcome to your subscription manager !</h1>
      {subscriptions.map((s) => {
        return <p key={s.name}>{s.name}</p>;
      })}
    </div>
  );
}

export default App;
