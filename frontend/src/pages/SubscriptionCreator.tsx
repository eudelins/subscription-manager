import { useNavigate } from 'react-router-dom';
import { SUBSCRIPTION_MANAGER_ROUTE } from 'routes/routes';

import { createSubscription } from 'services/subscriptions';
import SubscriptionForm from 'components/SubscriptionForm';

function SubscriptionCreator() {
  const navigate = useNavigate();
  const onFinish = async (formValues: any) => {
    const success = await createSubscription(
      formValues.name,
      formValues.price,
      formValues.status,
      formValues.brandId,
      formValues.categoriesId
    );
    if (success) {
      navigate(SUBSCRIPTION_MANAGER_ROUTE);
    }
  };

  return <SubscriptionForm isDisabled={false} onFinish={onFinish} />;
}

export default SubscriptionCreator;
