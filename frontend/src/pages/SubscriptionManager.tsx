import { useState, useEffect } from 'react';

import { getAllSubscriptions } from '../services/subscriptions';
import Subscriptions from '../interfaces/subscriptions/subscription.interface';

function SubscriptionManager() {
  const [subscriptions, setSubscriptions] = useState<Subscriptions[]>([]);

  useEffect(() => {
    getAllSubscriptions().then((res) => setSubscriptions(res));
  }, []);

  return (
    <>
      <p>SubscriptionManager</p>
      <div>
        {subscriptions.map((s) => {
          return <p key={s.name}>{s.name}</p>;
        })}
      </div>
    </>
  );
}

export default SubscriptionManager;
