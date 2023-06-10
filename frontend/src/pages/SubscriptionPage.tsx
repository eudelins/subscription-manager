import { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import { getSubscriptionById } from 'services/subscriptions';
import Subscription from 'interfaces/subscriptions/subscription.interface';
import { Row } from 'antd';
import DeleteSubscriptionsButton from 'components/buttons/DeleteSubscriptionButton';

function SubscriptionManager() {
  const { id = '' } = useParams();
  const [subscription, setSubscription] = useState<Subscription>();

  useEffect(() => {
    getSubscriptionById(id).then((res) => setSubscription(res));
  }, []);

  return (
    <>
      <Row justify="end">
        {subscription && <DeleteSubscriptionsButton subscription={subscription} />}
      </Row>
      <p>Id {subscription?.id}</p>
      <p>BrandId {subscription?.brandId}</p>
      <p>Name {subscription?.name}</p>
      <p>Price {subscription?.price}</p>
      <p>Stat {subscription?.status.toString()}</p>
      <p>Cat {subscription?.categoriesId}</p>
    </>
  );
}

export default SubscriptionManager;
